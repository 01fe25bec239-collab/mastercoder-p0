/* PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
 *
 * Native hot path for live_ledger.collect (full5 monitor and prestage capacity child).
 *
 * scan(roots, ...):  pass 1 over ALL roots in live_ledger order. Roots and `exact` (known-writer)
 *                    paths use lstat; every other child's metadata comes from its parent's no-follow
 *                    getattrlistbulk enumeration.
 * restat(paths, ...): pass 2, run only after pass 1 completed: one fresh lstat per path.
 * json_sha256(obj): sha256 hex of json.dumps(obj, sort_keys=True).encode(), streamed without the text.
 *
 * r2: `jobs` worker threads (the phase's existing jobs limit) perform ONLY the syscalls
 * (open/getattrlistbulk/close, lstat) into private C memory without the GIL. Everything that
 * decides anything runs afterwards, serially, under the GIL and in live_ledger order:
 * check() (the order, messages and integer conversions of live_ledger._stat/_stamp), the
 * pinned/None rows, and fault rows. A path's own LedgerError ends the returned list at that
 * path's position (pre-order in pass 1, observation order in pass 2), so the first reported
 * fault equals the reviewed serial implementation. Python then requires every unknown path's
 * pass-1 stamp to EQUAL its pass-2 lstat stamp, so accounting stays exact lstat values.
 * r4 (timing): pass-1 emission no longer waits for the whole walk. The GIL thread emits each
 * directory, in the same pre-order, as soon as its worker has published it (workers take the most
 * recently queued directory first, so they stay just ahead), and whenever it would otherwise wait
 * it hands the rows emitted so far to the optional `consume` callable (live_ledger's pass-1 record
 * step). Rows, order, checks and first fault are unchanged; the walk is still always joined before
 * scan returns or raises, and pass 2 still starts only after it.
 *
 * Read-only metadata: no content reads, no fd retained past one directory, no cache across calls.
 * Pending input on the optional control fd interrupts the walk (InterruptedError; partial work is
 * discarded) so the monitor services finish within its deadline.
 */
#define PY_SSIZE_T_CLEAN
#include <Python.h>
#include <CommonCrypto/CommonDigest.h>
#include <errno.h>
#include <fcntl.h>
#include <poll.h>
#include <pthread.h>
#include <signal.h>
#include <stdatomic.h>
#include <stdlib.h>
#include <string.h>
#include <sys/attr.h>
#include <sys/stat.h>
#include <sys/vnode.h>
#include <unistd.h>

#define MAX_JOBS 4

/* ---------- serial checks (GIL held) ---------- */

#define MODES 8

typedef struct {
    PyObject *uid, *device, *error, *out;
    /* r3: exact C values of uid/device when both are exact ints (fast paths only), and a few shared
     * mode ints. Ints are immutable: a reused object is the same value as a newly built one. */
    int fast; long long uidv, devv;
    int nmodes; mode_t modev[MODES]; PyObject *modes[MODES];
    PyObject *consume; Py_ssize_t consumed;  /* r4, scan only: optional callable; rows already handed to it */
} Ctx;

static void ctx_clear(Ctx *w) {
    for (int i = 0; i < w->nmodes; i++) Py_DECREF(w->modes[i]);
    w->nmodes = 0;
}

/* Exact C value of an exact int, when representable; 0 = unknown (the caller takes the exact path). */
static int as_ll(PyObject *o, long long *v) {
    int overflow = 0;
    if (!PyLong_CheckExact(o)) return 0;
    *v = PyLong_AsLongLongAndOverflow(o, &overflow);
    if (*v == -1 && PyErr_Occurred()) { PyErr_Clear(); return 0; }
    return !overflow;
}

static int fail_os(PyObject *error, const char *prefix, PyObject *path, int code) {
    errno = code;
    PyErr_SetFromErrnoWithFilenameObject(PyExc_OSError, path);
    PyObject *cause = PyErr_GetRaisedException();
    PyObject *msg = PyUnicode_FromFormat("%s%U", prefix, path);
    if (msg) {
        PyObject *exc = PyObject_CallOneArg(error, msg);
        Py_DECREF(msg);
        if (exc) {
            PyException_SetCause(exc, cause);  /* steals cause */
            PyErr_SetRaisedException(exc);
            return -1;
        }
    }
    Py_XDECREF(cause);
    return -1;
}

static int fail(PyObject *error, const char *prefix, PyObject *path) {
    PyObject *msg = path ? PyUnicode_FromFormat("%s%U", prefix, path) : PyUnicode_FromString(prefix);
    if (msg) {
        PyErr_SetObject(error, msg);
        Py_DECREF(msg);
    }
    return -1;
}

/* Same integer conversions as CPython 3.14 os.stat_result for these fields. */
static PyObject *time_ns(time_t sec, long nsec) {
    long long s = (long long)sec, out;
    if (!__builtin_mul_overflow(s, 1000000000LL, &out) && !__builtin_add_overflow(out, (long long)nsec, &out))
        return PyLong_FromLongLong(out);
    PyObject *a = PyLong_FromLongLong(s), *b = PyLong_FromLong(1000000000L), *c = PyLong_FromLong(nsec);
    PyObject *m = (a && b) ? PyNumber_Multiply(a, b) : NULL, *r = (m && c) ? PyNumber_Add(m, c) : NULL;
    Py_XDECREF(a); Py_XDECREF(b); Py_XDECREF(c); Py_XDECREF(m);
    return r;
}

/* r3: only the eight stamp fields (same names/types as struct stat), not a whole struct stat per
 * pass-1 entry / pass-2 slot: same values, far less memory. */
typedef struct {
    ino_t st_ino; off_t st_size; blkcnt_t st_blocks;
    struct timespec st_mtimespec, st_ctimespec;
    dev_t st_dev; uid_t st_uid; mode_t st_mode;
} Meta;

static void meta(Meta *m, const struct stat *st) {
    m->st_ino = st->st_ino; m->st_size = st->st_size; m->st_blocks = st->st_blocks;
    m->st_mtimespec = st->st_mtimespec; m->st_ctimespec = st->st_ctimespec;
    m->st_dev = st->st_dev; m->st_uid = st->st_uid; m->st_mode = st->st_mode;
}

static PyObject *mode_int(Ctx *w, mode_t mode) {
    for (int i = 0; i < w->nmodes; i++) if (w->modev[i] == mode) return Py_NewRef(w->modes[i]);
    PyObject *m = PyLong_FromLong((long)mode);
    if (m && w->nmodes < MODES) { w->modev[w->nmodes] = mode; w->modes[w->nmodes++] = Py_NewRef(m); }
    return m;
}

/* Same values and integer conversions as before; the device/uid ints are the caller's own exact int
 * objects when equal, and modes are shared, instead of one new int each per stamp. */
static PyObject *stamp(Ctx *w, const Meta *st) {
    PyObject *f[8];
    int negative_dev = 0;
#ifdef NODEV
    negative_dev = st->st_dev == NODEV;
#endif
    if (negative_dev) f[0] = PyLong_FromLongLong((long long)st->st_dev);
    else if (w->fast && w->devv >= 0 && (unsigned long long)st->st_dev == (unsigned long long)w->devv) f[0] = Py_NewRef(w->device);
    else f[0] = PyLong_FromUnsignedLongLong((unsigned long long)st->st_dev);
    f[1] = PyLong_FromUnsignedLongLong((unsigned long long)st->st_ino);
    f[2] = mode_int(w, st->st_mode);
    if (st->st_uid == (uid_t)-1) f[3] = PyLong_FromLong(-1);
    else if (w->fast && (long long)st->st_uid == w->uidv) f[3] = Py_NewRef(w->uid);
    else f[3] = PyLong_FromUnsignedLong(st->st_uid);
    f[4] = PyLong_FromLongLong((long long)st->st_size);
    f[5] = PyLong_FromLongLong((long long)st->st_blocks);
    f[6] = time_ns(st->st_mtimespec.tv_sec, st->st_mtimespec.tv_nsec);
    f[7] = time_ns(st->st_ctimespec.tv_sec, st->st_ctimespec.tv_nsec);
    PyObject *t = PyTuple_New(8);
    for (int i = 0; t && i < 8; i++) if (!f[i]) Py_CLEAR(t);
    if (!t) { for (int i = 0; i < 8; i++) Py_XDECREF(f[i]); return NULL; }
    for (int i = 0; i < 8; i++) PyTuple_SET_ITEM(t, i, f[i]);
    return t;
}

/* 1 only when check() would accept `st` AND build a stamp equal (Python tuple equality) to `old`,
 * decided on C values without building objects. 0 = not proven: the caller takes the exact
 * check()/RichCompare path unchanged, so every fault, message and unequal stamp is as before. */
static int same_stamp(const Ctx *w, const Meta *st, PyObject *old) {
    if (!w->fast || !PyTuple_CheckExact(old) || PyTuple_GET_SIZE(old) != 8) return 0;
#ifdef NODEV
    if (st->st_dev == NODEV) return 0;
#endif
    if (st->st_ino == 0 || st->st_size < 0 || st->st_blocks < 0 || st->st_uid == (uid_t)-1) return 0;  /* malformed_stat */
    if (!(S_ISREG(st->st_mode) || S_ISDIR(st->st_mode))) return 0;  /* symlink / unsupported_file_type */
    unsigned long long dev = (unsigned long long)st->st_dev, ino = (unsigned long long)st->st_ino;
    long long mt, ct;
    if (dev > LLONG_MAX || ino > LLONG_MAX ||
        __builtin_mul_overflow((long long)st->st_mtimespec.tv_sec, 1000000000LL, &mt) ||
        __builtin_add_overflow(mt, (long long)st->st_mtimespec.tv_nsec, &mt) ||
        __builtin_mul_overflow((long long)st->st_ctimespec.tv_sec, 1000000000LL, &ct) ||
        __builtin_add_overflow(ct, (long long)st->st_ctimespec.tv_nsec, &ct)) return 0;
    const long long want[8] = {(long long)dev, (long long)ino, (long long)st->st_mode, (long long)st->st_uid,
                               (long long)st->st_size, (long long)st->st_blocks, mt, ct};
    if (want[3] != w->uidv || want[0] != w->devv) return 0;  /* wrong_owner / unadmitted_separate_volume */
    for (int k = 0; k < 8; k++) {
        long long v;
        if (!as_ll(PyTuple_GET_ITEM(old, k), &v) || v != want[k]) return 0;
    }
    return 1;
}

/* _stamp then _stat's checks, in live_ledger order and with its messages. */
static PyObject *check(Ctx *w, PyObject *path, const Meta *st) {
    int negative_dev = 0;
#ifdef NODEV
    negative_dev = st->st_dev == NODEV;
#endif
    if (st->st_ino == 0 || st->st_size < 0 || st->st_blocks < 0 || negative_dev || st->st_uid == (uid_t)-1) {
        fail(w->error, "malformed_stat", NULL);
        return NULL;
    }
    if (S_ISLNK(st->st_mode)) { fail(w->error, "symlink:", path); return NULL; }
    if (!(S_ISREG(st->st_mode) || S_ISDIR(st->st_mode))) { fail(w->error, "unsupported_file_type:", path); return NULL; }
    PyObject *s = stamp(w, st);
    if (!s) return NULL;
    int same = PyObject_RichCompareBool(PyTuple_GET_ITEM(s, 3), w->uid, Py_EQ);
    if (same == 0) fail(w->error, "wrong_owner:", path);
    if (same == 1) {
        same = PyObject_RichCompareBool(PyTuple_GET_ITEM(s, 0), w->device, Py_EQ);
        if (same == 0) fail(w->error, "unadmitted_separate_volume:", path);
    }
    if (same == 1) return s;
    Py_DECREF(s);
    return NULL;
}

static int emit(Ctx *w, PyObject *path, PyObject *s) {
    PyObject *row = PyTuple_Pack(2, path, s);
    int r = row ? PyList_Append(w->out, row) : -1;
    Py_XDECREF(row);
    return r;
}

/* End the walk with (path, its own LedgerError) as the final row (1); other errors propagate (-1). */
static int stop(Ctx *w, PyObject *path) {
    if (!PyErr_ExceptionMatches(w->error)) return -1;
    PyObject *exc = PyErr_GetRaisedException();
    int r = emit(w, path, exc);
    Py_DECREF(exc);
    return r ? -1 : 1;
}

/* ---------- byte-string sets (built under the GIL, read-only in workers) ---------- */

typedef struct {
    char **keys;
    size_t *lens, *vals, cap;
} Set;

static uint64_t hash(const char *s, size_t n) {
    uint64_t h = 1469598103934665603ULL;
    for (size_t i = 0; i < n; i++) h = (h ^ (unsigned char)s[i]) * 1099511628211ULL;
    return h;
}

static void set_free(Set *s) {
    if (s->keys) for (size_t i = 0; i < s->cap; i++) free(s->keys[i]);
    free(s->keys); free(s->lens); free(s->vals);
    memset(s, 0, sizeof *s);
}

/* Returns stored value + 1 if present, else 0. */
static size_t set_get(const Set *s, const char *k, size_t n) {
    if (!s->cap) return 0;
    for (size_t i = hash(k, n) & (s->cap - 1);; i = (i + 1) & (s->cap - 1)) {
        if (!s->keys[i]) return 0;
        if (s->lens[i] == n && !memcmp(s->keys[i], k, n)) return s->vals[i] + 1;
    }
}

static int set_put(Set *s, const char *k, size_t n, size_t v) {
    for (size_t i = hash(k, n) & (s->cap - 1);; i = (i + 1) & (s->cap - 1)) {
        if (!s->keys[i]) {
            if (!(s->keys[i] = malloc(n + 1))) return -1;
            memcpy(s->keys[i], k, n); s->keys[i][n] = 0;
            s->lens[i] = n; s->vals[i] = v;
            return 0;
        }
        if (s->lens[i] == n && !memcmp(s->keys[i], k, n)) return 0;  /* first value wins */
    }
}

/* Build from an iterable of str (value = position). */
static int set_build(Set *s, PyObject *items) {
    memset(s, 0, sizeof *s);
    PyObject *seq = PySequence_Fast(items, "metawalk_arguments");
    if (!seq) return -1;
    Py_ssize_t n = PySequence_Fast_GET_SIZE(seq);
    size_t cap = 16;
    while (cap < (size_t)n * 2 + 16) cap <<= 1;
    s->cap = cap;
    s->keys = calloc(cap, sizeof *s->keys); s->lens = calloc(cap, sizeof *s->lens); s->vals = calloc(cap, sizeof *s->vals);
    if (!s->keys || !s->lens || !s->vals) { Py_DECREF(seq); set_free(s); PyErr_NoMemory(); return -1; }
    for (Py_ssize_t i = 0; i < n; i++) {
        PyObject *item = PySequence_Fast_GET_ITEM(seq, i);
        if (!PyUnicode_CheckExact(item)) { PyErr_SetString(PyExc_TypeError, "metawalk_arguments"); goto bad; }
        PyObject *raw = PyUnicode_EncodeFSDefault(item);
        if (!raw) goto bad;
        int r = set_put(s, PyBytes_AS_STRING(raw), (size_t)PyBytes_GET_SIZE(raw), (size_t)i);
        Py_DECREF(raw);
        if (r) { PyErr_NoMemory(); goto bad; }
    }
    Py_DECREF(seq);
    return 0;
bad:
    Py_DECREF(seq);
    set_free(s);
    return -1;
}

/* ---------- pass 1 tree (filled by workers without the GIL) ---------- */

enum { NORMAL, PINNED, SKIP };

typedef struct Node Node;
typedef struct {
    char *path; size_t len;
    Meta st;
    int err;          /* errno for this entry (bulk ATTR_CMN_ERROR or exact lstat) */
    int kind;
    int malformed;
    Node *child;      /* set when the worker queued this directory for enumeration */
} Entry;

struct Node {
    char *path; size_t len;
    int owns;         /* root nodes own path; a child borrows its parent Entry's path (freed after it) */
    size_t root;      /* index of the root this directory belongs to */
    Entry *entries; size_t n, cap;
    int err;          /* open/getattrlistbulk errno: unreadable_directory */
    int malformed;    /* record framing error: malformed_stat */
    atomic_int done;  /* r4: set (under the pool lock) once its worker finished enumerating it */
};

typedef struct {
    Set pinned, exact, roots;
    uid_t uid; dev_t dev; int match;  /* pre-check for descent only; check() decides */
    int control;
    pthread_mutex_t lock; pthread_cond_t wake;
    Node **queue; size_t tail, cap;  /* r4: LIFO, so workers walk just ahead of pre-order emission */
    int active, failed, live;        /* live: started worker threads not yet exited */
    atomic_int abort;   /* 1 = control input pending, 2 = out of memory, 3 = r4: `consume` raised */
    atomic_long count;
} Pool;

static void node_free(Node *n) {
    if (!n) return;
    for (size_t i = 0; i < n->n; i++) { node_free(n->entries[i].child); free(n->entries[i].path); }
    free(n->entries); if (n->owns) free(n->path); free(n);
}

static Node *node_new(const char *path, size_t len, size_t root) {
    Node *n = calloc(1, sizeof *n);
    if (!n || !(n->path = malloc(len + 1))) { free(n); return NULL; }
    memcpy(n->path, path, len); n->path[len] = 0; n->len = len; n->root = root; n->owns = 1;
    return n;
}

static Node *node_child(Entry *e, size_t root) {  /* e->path outlives the child: node_free frees it after */
    Node *n = calloc(1, sizeof *n);
    if (n) { n->path = e->path; n->len = e->len; n->root = root; }
    return n;
}

/* 1 if control input (or EOF/error state) is pending; EINTR is retried, never treated as input. */
static int pending(int fd) {
    if (fd < 0) return 0;
    struct pollfd pfd = {.fd = fd, .events = POLLIN};
    int r;
    while ((r = poll(&pfd, 1, 0)) < 0 && errno == EINTR) {}
    return r != 0;
}

static void ticker(Pool *p) {
    if ((atomic_fetch_add(&p->count, 1) & 1023) != 1023) return;
    if (pending(p->control)) atomic_store(&p->abort, 1);
}

#define CMN (ATTR_CMN_RETURNED_ATTRS | ATTR_CMN_NAME | ATTR_CMN_DEVID | ATTR_CMN_OBJTYPE | ATTR_CMN_MODTIME | \
             ATTR_CMN_CHGTIME | ATTR_CMN_OWNERID | ATTR_CMN_ACCESSMASK | ATTR_CMN_FILEID)
static const struct attrlist BULK = {.bitmapcount = ATTR_BIT_MAP_COUNT, .commonattr = CMN | ATTR_CMN_ERROR,
                                     .dirattr = ATTR_DIR_ALLOCSIZE | ATTR_DIR_DATALENGTH,
                                     .fileattr = ATTR_FILE_ALLOCSIZE | ATTR_FILE_DATALENGTH};

static mode_t format(fsobj_type_t type) {
    switch (type) {
    case VREG: return S_IFREG;
    case VDIR: return S_IFDIR;
    case VLNK: return S_IFLNK;
    case VSOCK: return S_IFSOCK;
    case VFIFO: return S_IFIFO;
    case VCHR: return S_IFCHR;
    case VBLK: return S_IFBLK;
    default: return 0;
    }
}

#define TAKE(dst) do { if (p + sizeof(dst) > end) return -1; memcpy(&(dst), p, sizeof(dst)); p += sizeof(dst); } while (0)
/* After the name: a short record is this entry's malformed_stat (kind decides whether it matters). */
#define TAKEM(dst) do { if (p + sizeof(dst) > end) { e->malformed = 1; return 0; } memcpy(&(dst), p, sizeof(dst)); p += sizeof(dst); } while (0)

/* One bulk record -> entry fields. -1: framing error (malformed_stat). */
static int parse(char *p, char *end, Entry *e, const char **name) {
    attribute_set_t got;
    attrreference_t ref;
    uint32_t error = 0;
    TAKE(got);
    if (got.commonattr & ATTR_CMN_ERROR) TAKE(error);
    char *field = p;
    TAKE(ref);
    char *text = field + ref.attr_dataoffset;
    if (text < field || ref.attr_length < 1 || text + ref.attr_length > end || !memchr(text, 0, ref.attr_length)) return -1;
    *name = text;
    if (error) { e->err = (int)error; return 0; }
    if ((got.commonattr & CMN) != CMN) { e->malformed = 1; return 0; }
    fsobj_type_t type;
    uint32_t mask;
    uint64_t id;
    off_t alloc = 0, size = 0;
    TAKEM(e->st.st_dev);
    TAKEM(type);
    TAKEM(e->st.st_mtimespec);
    TAKEM(e->st.st_ctimespec);
    TAKEM(e->st.st_uid);
    TAKEM(mask);
    TAKEM(id);
    e->st.st_ino = id;
    e->st.st_mode = format(type) | (mode_t)(mask & 07777);
    /* Sizes are required for the two admitted kinds; other kinds fail on type in check(). */
    u_int32_t want = type == VDIR ? (ATTR_DIR_ALLOCSIZE | ATTR_DIR_DATALENGTH) : (ATTR_FILE_ALLOCSIZE | ATTR_FILE_DATALENGTH);
    u_int32_t have = type == VDIR ? got.dirattr : got.fileattr;
    if ((have & want) == want) {
        TAKEM(alloc);
        TAKEM(size);
    } else if (type == VDIR || type == VREG) {
        e->malformed = 1;
        return 0;
    }
    /* vn_stat: st_blocks = roundup(total allocation, 512) / 512. */
    e->st.st_size = size;
    e->st.st_blocks = alloc < 0 ? -1 : (alloc + 511) / 512;
    return 0;
}

static int push(Pool *p, Node *n) {  /* lock held */
    if (p->tail == p->cap) {
        size_t cap = p->cap ? p->cap * 2 : 1024;
        Node **q = realloc(p->queue, cap * sizeof *q);
        if (!q) return -1;
        p->queue = q; p->cap = cap;
    }
    p->queue[p->tail++] = n;
    return 0;
}

/* Enumerate one directory; returns child directories to queue (count in *found). */
static void enumerate(Pool *p, Node *node, Node ***found, size_t *nfound, size_t *foundcap) {
    static __thread char buffer[128 * 1024];
    *nfound = 0;
    int fd = open(node->path, O_RDONLY | O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC);
    if (fd < 0) { node->err = errno ? errno : EIO; return; }
    int n = 0;
    while (!atomic_load(&p->abort) && (n = getattrlistbulk(fd, (struct attrlist *)&BULK, buffer, sizeof buffer, 0)) > 0) {
        char *q = buffer, *end = buffer + sizeof buffer;
        uint32_t length = 0;
        /* r3: room for exactly this call's records (at most one entry each), not a 16-entry minimum
         * doubled per directory: most directories here hold one or two entries. */
        if ((size_t)n > node->cap - node->n) {
            size_t cap = node->n + (size_t)n;
            Entry *grown = realloc(node->entries, cap * sizeof *grown);
            if (!grown) { atomic_store(&p->abort, 2); break; }
            node->entries = grown; node->cap = cap;
        }
        for (int i = 0; i < n; i++, q += length) {  /* advance in the header: `continue` must not skip it */
            if (q + sizeof length > end) { node->malformed = 1; break; }
            memcpy(&length, q, sizeof length);
            if (length < sizeof length || q + length > end) { node->malformed = 1; break; }
            Entry *e = &node->entries[node->n];
            memset(e, 0, sizeof *e);
            const char *name = NULL;
            if (parse(q + sizeof length, q + length, e, &name)) { node->malformed = 1; break; }
            size_t nl = strlen(name);
            size_t jl = node->len;  /* live_ledger._child_visits: path.rstrip('/') + '/' + name */
            while (jl > 0 && node->path[jl - 1] == '/') jl--;
            if (!(e->path = malloc(jl + 1 + nl + 1))) { atomic_store(&p->abort, 2); break; }
            memcpy(e->path, node->path, jl); e->path[jl] = '/';
            memcpy(e->path + jl + 1, name, nl + 1);
            e->len = jl + 1 + nl;
            node->n++;
            ticker(p);
            /* live_ledger.visit's `path in observations`: an earlier root's own tree covers it. */
            size_t root = set_get(&p->roots, e->path, e->len);
            if (root && root - 1 < node->root) { e->kind = SKIP; continue; }
            if (set_get(&p->pinned, e->path, e->len)) { e->kind = PINNED; continue; }
            if (set_get(&p->exact, e->path, e->len)) {
                struct stat st;
                memset(&e->st, 0, sizeof e->st); e->err = 0; e->malformed = 0;
                if (lstat(e->path, &st) != 0) e->err = errno ? errno : EIO; else meta(&e->st, &st);
            } else if (!e->err && e->malformed) {
                node->malformed = 1;  /* r1 raised malformed_stat while listing: before any later entry */
                break;
            }
            if (e->err || e->malformed || !S_ISDIR(e->st.st_mode) || !p->match ||
                e->st.st_uid != p->uid || e->st.st_dev != p->dev) continue;
            Node *child = node_child(e, node->root);
            if (child && *nfound == *foundcap) {
                size_t cap = *foundcap ? *foundcap * 2 : 64;
                Node **grown = realloc(*found, cap * sizeof **found);
                if (grown) { *found = grown; *foundcap = cap; }
            }
            if (!child || *nfound == *foundcap) { node_free(child); atomic_store(&p->abort, 2); break; }
            (*found)[(*nfound)++] = child;
            e->child = child;
        }
        if (node->malformed) break;
    }
    if (!node->malformed && n < 0 && !atomic_load(&p->abort)) node->err = errno ? errno : EIO;
    close(fd);
}

static void *worker(void *arg) {
    Pool *p = arg;
    Node **found = NULL;
    size_t foundcap = 0;
    pthread_mutex_lock(&p->lock);
    for (;;) {
        while (!p->tail && p->active && !atomic_load(&p->abort)) pthread_cond_wait(&p->wake, &p->lock);
        if (atomic_load(&p->abort) || !p->tail) break;
        Node *node = p->queue[--p->tail];
        p->active++;
        pthread_mutex_unlock(&p->lock);
        size_t nfound = 0;
        enumerate(p, node, &found, &nfound, &foundcap);
        pthread_mutex_lock(&p->lock);
        for (size_t i = nfound; i > 0; i--)  /* reversed: the first child is taken next */
            if (push(p, found[i - 1])) { atomic_store(&p->abort, 2); break; }
        atomic_store(&node->done, 1);  /* after every write to node: emission may now read it */
        p->active--;
        pthread_cond_broadcast(&p->wake);
    }
    pthread_cond_broadcast(&p->wake);
    pthread_mutex_unlock(&p->lock);
    free(found);
    return NULL;
}

static void *worker_thread(void *arg) {
    Pool *p = arg;
    worker(p);
    pthread_mutex_lock(&p->lock);
    p->live--;
    pthread_cond_broadcast(&p->wake);
    pthread_mutex_unlock(&p->lock);
    return NULL;
}

/* r4: 1 once `node` is enumerated and no abort is set; 0 if the walk was aborted (or, never expected,
 * ended without it). Instead of idling, first hands any newly emitted rows to `consume` (GIL held);
 * if that raises, the walk is aborted (3) and its exception is kept. Then waits without the GIL. An
 * aborting worker sets abort before it publishes the node it was enumerating, so a partially
 * enumerated node is never read. */
static int ready(Ctx *w, Pool *p, Node *node) {
    if (!atomic_load(&node->done) && !atomic_load(&p->abort) && w->consume && PyList_GET_SIZE(w->out) > w->consumed) {
        w->consumed = PyList_GET_SIZE(w->out);
        PyObject *r = PyObject_CallOneArg(w->consume, w->out);
        if (!r) {
            int none = 0;
            atomic_compare_exchange_strong(&p->abort, &none, 3);
            return 0;
        }
        Py_DECREF(r);
    }
    if (atomic_load(&node->done) && !atomic_load(&p->abort)) return 1;
    int ok;
    Py_BEGIN_ALLOW_THREADS
    pthread_mutex_lock(&p->lock);
    while (!atomic_load(&node->done) && !atomic_load(&p->abort) && p->live) pthread_cond_wait(&p->wake, &p->lock);
    ok = atomic_load(&node->done) && !atomic_load(&p->abort);
    pthread_mutex_unlock(&p->lock);
    Py_END_ALLOW_THREADS
    return ok;
}

/* Pre-order emission (GIL): same rows, order and first fault as the serial walker.
 * r4: runs while the workers walk; each node is read only once ready(). */
static int emit_node(Ctx *w, Pool *p, Node *node) {
    if (!ready(w, p, node)) {
        if (!PyErr_Occurred() && !atomic_load(&p->abort)) PyErr_SetString(PyExc_RuntimeError, "metawalk_incomplete_walk");
        return -1;  /* an abort's own error is decided after the join */
    }
    PyObject *dir = PyUnicode_DecodeFSDefaultAndSize(node->path, (Py_ssize_t)node->len);
    if (!dir) return -1;
    if (node->err) {
        int r = (fail_os(w->error, "unreadable_directory:", dir, node->err), stop(w, dir));
        Py_DECREF(dir);
        return r;
    }
    if (node->malformed) {
        int r = (fail(w->error, "malformed_stat", NULL), stop(w, dir));
        Py_DECREF(dir);
        return r;
    }
    Py_DECREF(dir);
    for (size_t i = 0; i < node->n; i++) {
        Entry *e = &node->entries[i];
        if (e->kind == SKIP) continue;
        PyObject *path = PyUnicode_DecodeFSDefaultAndSize(e->path, (Py_ssize_t)e->len);
        if (!path) return -1;
        int r;
        if (e->kind == PINNED) {
            r = emit(w, path, Py_None);
        } else if (e->err) {
            r = (fail_os(w->error, "missing_or_unreadable:", path, e->err), stop(w, path));
        } else if (e->malformed) {
            r = (fail(w->error, "malformed_stat", NULL), stop(w, path));
        } else {
            PyObject *s = check(w, path, &e->st);
            r = s ? emit(w, path, s) : stop(w, path);
            Py_XDECREF(s);
            if (!r && S_ISDIR(e->st.st_mode)) {
                /* check() passed, so the worker's descent pre-check must have queued it. */
                r = e->child ? emit_node(w, p, e->child) : fail(w->error, "metawalk_descent_mismatch:", path);
            }
        }
        Py_DECREF(path);
        if (r) return r;
    }
    return 0;
}

static int control_fd(PyObject *value) {
    if (value == Py_None) return -1;
    long fd = PyLong_Check(value) ? PyLong_AsLong(value) : -1;
    if (fd < 0 || fd > 1 << 20) {
        if (!PyErr_Occurred()) PyErr_SetString(PyExc_TypeError, "metawalk_arguments");
        return -2;
    }
    return (int)fd;
}

static int jobs_arg(PyObject *value) {
    long jobs = PyLong_Check(value) ? PyLong_AsLong(value) : 0;
    if (jobs < 1 || jobs > MAX_JOBS) {
        if (!PyErr_Occurred()) PyErr_SetString(PyExc_TypeError, "metawalk_arguments");
        return -1;
    }
    return (int)jobs;
}

static int common(PyObject *const *args, Ctx *w) {
    w->uid = args[0]; w->device = args[1]; w->error = args[2];
    w->nmodes = 0;
    w->fast = as_ll(w->uid, &w->uidv) && as_ll(w->device, &w->devv);
    if (!PyLong_Check(w->uid) || !PyLong_Check(w->device) || !PyType_Check(w->error) ||
        !PyType_IsSubtype((PyTypeObject *)w->error, (PyTypeObject *)PyExc_Exception)) {
        PyErr_SetString(PyExc_TypeError, "metawalk_arguments");
        return -1;
    }
    return 0;
}

static int interrupted(int abort) {
    if (abort == 2) { PyErr_NoMemory(); return -1; }
    if (abort == 1) { PyErr_SetString(PyExc_InterruptedError, "collection_interrupted_by_control"); return -1; }
    return PyErr_CheckSignals();
}

/* scan(roots, pinned, exact, uid, device, error, control, jobs) -> [(path, stamp-or-None-or-fault), ...] */
static PyObject *scan(PyObject *Py_UNUSED(self), PyObject *const *args, Py_ssize_t n) {
    Ctx w;
    Pool p;
    memset(&p, 0, sizeof p);
    w.consume = n == 9 && args[8] != Py_None ? args[8] : NULL; w.consumed = 0;
    if ((n != 8 && n != 9) || !PyTuple_CheckExact(args[0]) || !PyAnySet_Check(args[1]) || !PyAnySet_Check(args[2]) ||
        (w.consume && !PyCallable_Check(w.consume)) || common(args + 3, &w))
        { if (!PyErr_Occurred()) PyErr_SetString(PyExc_TypeError, "metawalk_arguments"); return NULL; }
    int jobs = jobs_arg(args[7]);
    if (jobs < 0 || (p.control = control_fd(args[6])) == -2) return NULL;
    PyObject *roots = args[0];
    Py_ssize_t nroots = PyTuple_GET_SIZE(roots);
    unsigned long long dev = PyLong_AsUnsignedLongLong(w.device);
    unsigned long uid = PyLong_AsUnsignedLong(w.uid);
    p.match = !PyErr_Occurred() && dev == (unsigned long long)(dev_t)dev && uid == (unsigned long)(uid_t)uid;
    PyErr_Clear();
    p.dev = (dev_t)dev; p.uid = (uid_t)uid;
    if (set_build(&p.pinned, args[1]) || set_build(&p.exact, args[2]) || set_build(&p.roots, roots)) {
        set_free(&p.pinned); set_free(&p.exact); return NULL;
    }
    w.out = PyList_New(0);
    Node **tops = calloc((size_t)nroots + 1, sizeof *tops);
    PyObject **firsts = calloc((size_t)nroots + 1, sizeof *firsts);  /* root row (stamp/None/fault) */
    int result = (w.out && tops && firsts) ? 0 : (PyErr_NoMemory(), -1);

    /* Roots: lstat + check serially (GIL), in order; a root inside an earlier root is covered. */
    for (Py_ssize_t i = 0; !result && i < nroots; i++) {
        PyObject *root = PyTuple_GET_ITEM(roots, i);
        PyObject *raw = PyUnicode_CheckExact(root) ? PyUnicode_EncodeFSDefault(root) : NULL;
        if (!raw) { if (!PyErr_Occurred()) PyErr_SetString(PyExc_TypeError, "metawalk_arguments"); result = -1; break; }
        const char *s = PyBytes_AS_STRING(raw);
        size_t len = (size_t)PyBytes_GET_SIZE(raw), covered = 0;
        for (Py_ssize_t j = 0; j < i && !covered; j++) {
            if (!tops[j] && !firsts[j]) continue;
            PyObject *other = PyUnicode_EncodeFSDefault(PyTuple_GET_ITEM(roots, j));
            if (!other) { result = -1; break; }
            size_t ol = (size_t)PyBytes_GET_SIZE(other);
            const char *os_ = PyBytes_AS_STRING(other);
            while (ol > 1 && os_[ol - 1] == '/') ol--;
            covered = (ol == len && !memcmp(os_, s, len)) ||
                      (ol < len && !memcmp(os_, s, ol) && (s[ol] == '/' || (ol == 1 && os_[0] == '/')));
            Py_DECREF(other);
        }
        if (result || covered) { Py_DECREF(raw); continue; }
        if (set_get(&p.pinned, s, len)) {
            firsts[i] = Py_NewRef(Py_None);
        } else {
            struct stat st;
            Meta m;
            if (lstat(s, &st) != 0) {
                fail_os(w.error, "missing_or_unreadable:", root, errno);
            } else {
                meta(&m, &st);
                PyObject *checked = check(&w, root, &m);
                if (checked && S_ISDIR(st.st_mode) && !(tops[i] = node_new(s, len, (size_t)i))) {
                    Py_DECREF(checked); checked = NULL; PyErr_NoMemory();
                }
                firsts[i] = checked;
            }
            if (!firsts[i]) {
                if (!PyErr_ExceptionMatches(w.error)) { Py_DECREF(raw); result = -1; break; }
                firsts[i] = PyErr_GetRaisedException();
                Py_DECREF(raw);
                break;  /* nothing after this root's own fault is emitted */
            }
        }
        Py_DECREF(raw);
    }

    /* Pass 1 enumeration: worker threads, syscalls only, no GIL. r4: meanwhile this thread emits in
     * live_ledger order (each node once its worker published it) and feeds `consume`; the walk always
     * runs to completion or abort and is joined before anything is returned or raised. */
    if (!result) {
        pthread_mutex_init(&p.lock, NULL);
        pthread_cond_init(&p.wake, NULL);
        for (Py_ssize_t i = nroots; i > 0; i--)  /* LIFO: root 0 is taken first */
            if (tops[i - 1] && push(&p, tops[i - 1])) { PyErr_NoMemory(); result = -1; break; }
        if (!result) {
            pthread_t threads[MAX_JOBS];
            int started = 0;
            if (pending(p.control)) atomic_store(&p.abort, 1);  /* even a tiny tree services control first */
            sigset_t all, old;
            sigfillset(&all);
            pthread_mutex_lock(&p.lock);  /* no worker can exit before `live` counts it */
            pthread_sigmask(SIG_BLOCK, &all, &old);  /* workers inherit: signals stay on the main thread */
            for (int t = 0; t < jobs; t++)
                if (!pthread_create(&threads[started], NULL, worker_thread, &p)) started++;
            pthread_sigmask(SIG_SETMASK, &old, NULL);
            p.live = started;
            pthread_mutex_unlock(&p.lock);
            if (!started) worker(&p);  /* never skip the walk */

            /* Emission in live_ledger order. */
            for (Py_ssize_t i = 0; !result && i < nroots; i++) {
                if (!firsts[i]) continue;
                PyObject *root = PyTuple_GET_ITEM(roots, i);
                int r = emit(&w, root, firsts[i]);
                if (!r && PyExceptionInstance_Check(firsts[i])) break;
                if (!r && tops[i]) r = emit_node(&w, &p, tops[i]);
                if (r < 0) result = -1;
                if (r) break;
            }

            PyObject *raised = PyErr_GetRaisedException();  /* emission's or consume's own error, if any */
            Py_BEGIN_ALLOW_THREADS
            for (int t = 0; t < started; t++) pthread_join(threads[t], NULL);
            Py_END_ALLOW_THREADS
            /* A queued node never enumerated (abort) cannot be emitted: the walk is discarded. As before,
             * a control/memory abort or a pending signal takes precedence over any emission result;
             * an abort caused by `consume` raises consume's exception. */
            int abort = atomic_load(&p.abort);
            if (abort == 3) {
                if (raised) PyErr_SetRaisedException(raised);
                else PyErr_SetString(PyExc_RuntimeError, "metawalk_consume_abort");
            } else if (interrupted(abort)) { Py_XDECREF(raised); result = -1; }
            else if (raised) PyErr_SetRaisedException(raised);
        }
        pthread_cond_destroy(&p.wake);
        pthread_mutex_destroy(&p.lock);
    }

    for (Py_ssize_t i = 0; i < nroots; i++) { if (firsts) Py_XDECREF(firsts[i]); if (tops) node_free(tops[i]); }
    free(tops); free(firsts); free(p.queue);
    set_free(&p.pinned); set_free(&p.exact); set_free(&p.roots);
    ctx_clear(&w);
    if (result) { Py_XDECREF(w.out); return NULL; }
    return w.out;
}

/* ---------- pass 2 ---------- */

typedef struct {
    char **paths; Meta *st; int *err;
    size_t from, to;
    int control; atomic_int *abort, *finished;
} Slice;

static void *restat_worker(void *arg) {
    Slice *s = arg;
    for (size_t i = s->from; i < s->to; i++) {
        if (((i - s->from) & 1023) == 0 && s->control >= 0) {  /* slice start, then every 1024 */
            if (atomic_load(s->abort)) return NULL;
            if (pending(s->control)) { atomic_store(s->abort, 1); return NULL; }
        }
        struct stat st;
        if (s->paths[i] && lstat(s->paths[i], &st) != 0) s->err[i] = errno ? errno : EIO;
        else if (s->paths[i]) meta(&s->st[i], &st);
    }
    return NULL;
}

static void *restat_thread(void *arg) {
    restat_worker(arg);
    atomic_fetch_add(((Slice *)arg)->finished, 1);
    return NULL;
}

/* restat(paths, pinned, uid, device, error, control, jobs[, initial[, background]]) -> [stamp-or-None, ...]
 * At the first path whose own check fails, the list ends with that LedgerError instance.
 * With `initial` (the pass-1 stamps, one per path), a checked fresh stamp that EQUALS its initial
 * stamp is returned as that same initial object; unequal stamps are returned as built, unchanged.
 * r3: an optional `background` callable is called repeatedly, on this (GIL-holding) thread only, while
 * the worker threads lstat; it stops at the first false result or once every worker has finished.
 * It must do pure-CPU work (no syscalls), so at most `jobs` threads ever do file-system calls. Its
 * exception is raised after every worker has been joined; no result list is then returned. */
static PyObject *restat(PyObject *Py_UNUSED(self), PyObject *const *args, Py_ssize_t n) {
    Ctx w;
    PyObject *initial = n >= 8 ? args[7] : Py_None, *background = n == 9 ? args[8] : NULL;
    if ((n != 7 && n != 8 && n != 9) || !PyTuple_CheckExact(args[0]) || !PyAnySet_Check(args[1]) || common(args + 2, &w) ||
        (initial != Py_None && (!PyTuple_CheckExact(initial) || PyTuple_GET_SIZE(initial) != PyTuple_GET_SIZE(args[0]))) ||
        (background && !PyCallable_Check(background)))
        { if (!PyErr_Occurred()) PyErr_SetString(PyExc_TypeError, "metawalk_arguments"); return NULL; }
    int jobs = jobs_arg(args[6]), control = control_fd(args[5]);
    if (jobs < 0 || control == -2) return NULL;
    PyObject *paths = args[0];
    size_t count = (size_t)PyTuple_GET_SIZE(paths);
    PyObject *raws = PyList_New((Py_ssize_t)count);
    char **cpaths = calloc(count + 1, sizeof *cpaths);
    Meta *st = calloc(count + 1, sizeof *st);
    int *err = calloc(count + 1, sizeof *err);
    PyObject *out = NULL;
    atomic_int abort = 0, finished = 0;
    int failed = 0;
    if (!raws || !cpaths || !st || !err) { PyErr_NoMemory(); goto done; }
    for (size_t i = 0; i < count; i++) {
        PyObject *path = PyTuple_GET_ITEM(paths, (Py_ssize_t)i);
        int is_pinned = PyUnicode_CheckExact(path) ? PySet_Contains(args[1], path) : -1;
        if (is_pinned < 0) { if (!PyErr_Occurred()) PyErr_SetString(PyExc_TypeError, "metawalk_arguments"); goto done; }
        PyObject *raw = is_pinned ? Py_NewRef(Py_None) : PyUnicode_EncodeFSDefault(path);
        if (!raw) goto done;
        PyList_SET_ITEM(raws, (Py_ssize_t)i, raw);
        cpaths[i] = is_pinned ? NULL : PyBytes_AS_STRING(raw);
    }
    Slice slices[MAX_JOBS];
    pthread_t threads[MAX_JOBS];
    int started = 0;
    sigset_t all, old;
    sigfillset(&all);
    pthread_sigmask(SIG_BLOCK, &all, &old);  /* workers inherit: signals stay on the main thread */
    for (int t = 0; t < jobs; t++) {
        slices[t] = (Slice){cpaths, st, err, count * (size_t)t / (size_t)jobs, count * (size_t)(t + 1) / (size_t)jobs, control, &abort, &finished};
        if (!pthread_create(&threads[started], NULL, restat_thread, &slices[t])) started++;
        else restat_worker(&slices[t]);  /* never skip a slice; each slice runs exactly once */
    }
    pthread_sigmask(SIG_SETMASK, &old, NULL);
    while (background && atomic_load(&finished) < started) {  /* workers never touch Python objects */
        PyObject *more = PyObject_CallNoArgs(background);
        int go = more ? PyObject_IsTrue(more) : -1;
        Py_XDECREF(more);
        if (go < 0) failed = 1;
        if (go <= 0) break;
    }
    Py_BEGIN_ALLOW_THREADS
    for (int t = 0; t < started; t++) pthread_join(threads[t], NULL);
    Py_END_ALLOW_THREADS
    if (failed || interrupted(atomic_load(&abort))) goto done;
    if (!(out = PyList_New((Py_ssize_t)count))) goto done;
    for (size_t i = 0; i < count; i++) {
        PyObject *path = PyTuple_GET_ITEM(paths, (Py_ssize_t)i), *s;
        if (!cpaths[i]) {
            s = Py_NewRef(Py_None);
        } else if (initial != Py_None && !err[i] && same_stamp(&w, &st[i], PyTuple_GET_ITEM(initial, (Py_ssize_t)i))) {
            s = Py_NewRef(PyTuple_GET_ITEM(initial, (Py_ssize_t)i));  /* what check() + equality would return */
        } else {
            s = err[i] ? (fail_os(w.error, "missing_or_unreadable:", path, err[i]), NULL) : check(&w, path, &st[i]);
            if (!s) {
                if (!PyErr_ExceptionMatches(w.error)) { Py_CLEAR(out); goto done; }
                PyList_SET_ITEM(out, (Py_ssize_t)i, PyErr_GetRaisedException());
                if (PyList_SetSlice(out, (Py_ssize_t)i + 1, (Py_ssize_t)count, NULL)) Py_CLEAR(out);
                goto done;
            }
            if (initial != Py_None) {  /* Python tuple equality, exactly live_ledger's `current == initial` */
                PyObject *old = PyTuple_GET_ITEM(initial, (Py_ssize_t)i);
                int same = PyObject_RichCompareBool(s, old, Py_EQ);
                if (same < 0) { Py_DECREF(s); Py_CLEAR(out); goto done; }
                if (same) Py_SETREF(s, Py_NewRef(old));
            }
        }
        PyList_SET_ITEM(out, (Py_ssize_t)i, s);
    }
done:
    ctx_clear(&w);
    Py_XDECREF(raws); free(cpaths); free(st); free(err);
    return out;
}

/* ---------- settlement fingerprint digest ---------- */

/* Exactly the bytes of json.dumps(obj, sort_keys=True) (default ", " / ": " separators, ensure_ascii)
 * for exact tuple/list/str/int/None/bool values, fed to SHA-256 in 64 KiB pieces. Any other value
 * raises TypeError before a digest is returned; live_ledger then lets the json module decide. */
typedef struct { CC_SHA256_CTX ctx; size_t n; unsigned char buf[1 << 16]; } Digest;

static void put(Digest *d, const void *p, size_t len) {
    if (len > sizeof d->buf - d->n) { CC_SHA256_Update(&d->ctx, d->buf, (CC_LONG)d->n); d->n = 0; }
    if (len > sizeof d->buf) {  /* CC_LONG is 32-bit: update in 64 KiB pieces so no byte is dropped */
        for (const unsigned char *q = p; len;) {
            size_t k = len < sizeof d->buf ? len : sizeof d->buf;
            CC_SHA256_Update(&d->ctx, q, (CC_LONG)k); q += k; len -= k;
        }
        return;
    }
    memcpy(d->buf + d->n, p, len); d->n += len;
}

/* CPython _json.c ascii_escape_unichar / S_CHAR, lowercase Py_hexdigits. */
static void put_char(Digest *d, Py_UCS4 c) {
    static const char hex[] = "0123456789abcdef";
    char out[12];
    size_t k = 0;
    if (c >= ' ' && c <= '~' && c != '\\' && c != '"') { out[k++] = (char)c; put(d, out, k); return; }
    out[k++] = '\\';
    switch (c) {
    case '\\': case '"': out[k++] = (char)c; break;
    case '\b': out[k++] = 'b'; break;
    case '\f': out[k++] = 'f'; break;
    case '\n': out[k++] = 'n'; break;
    case '\r': out[k++] = 'r'; break;
    case '\t': out[k++] = 't'; break;
    default:
        if (c >= 0x10000) {
            Py_UCS4 v = Py_UNICODE_HIGH_SURROGATE(c);
            out[k++] = 'u'; out[k++] = hex[(v >> 12) & 0xf]; out[k++] = hex[(v >> 8) & 0xf];
            out[k++] = hex[(v >> 4) & 0xf]; out[k++] = hex[v & 0xf];
            c = Py_UNICODE_LOW_SURROGATE(c);
            out[k++] = '\\';
        }
        out[k++] = 'u'; out[k++] = hex[(c >> 12) & 0xf]; out[k++] = hex[(c >> 8) & 0xf];
        out[k++] = hex[(c >> 4) & 0xf]; out[k++] = hex[c & 0xf];
    }
    put(d, out, k);
}

static int put_json(Digest *d, PyObject *o, int depth) {
    if (o == Py_None) { put(d, "null", 4); return 0; }
    if (o == Py_True) { put(d, "true", 4); return 0; }
    if (o == Py_False) { put(d, "false", 5); return 0; }
    if (PyLong_CheckExact(o)) {
        int overflow = 0;
        long long v = PyLong_AsLongLongAndOverflow(o, &overflow);
        if (v == -1 && PyErr_Occurred()) return -1;
        if (!overflow) {  /* the "%lld" text without snprintf: eight ints per metadata row */
            char text[24], *q = text + sizeof text;
            unsigned long long u = v < 0 ? 0ULL - (unsigned long long)v : (unsigned long long)v;
            do { *--q = (char)('0' + u % 10); u /= 10; } while (u);
            if (v < 0) *--q = '-';
            put(d, q, (size_t)(text + sizeof text - q));
            return 0;
        }
        PyObject *text = PyLong_Type.tp_repr(o);  /* what json uses for int */
        if (!text) return -1;
        Py_ssize_t k;
        const char *raw = PyUnicode_AsUTF8AndSize(text, &k);
        if (raw) put(d, raw, (size_t)k);
        Py_DECREF(text);
        return raw ? 0 : -1;
    }
    if (PyUnicode_CheckExact(o)) {
        Py_ssize_t len = PyUnicode_GET_LENGTH(o);
        put(d, "\"", 1);
        if (PyUnicode_IS_ASCII(o)) {
            const char *s = (const char *)PyUnicode_1BYTE_DATA(o);
            Py_ssize_t from = 0;
            for (Py_ssize_t i = 0; i < len; i++) {
                char c = s[i];
                if (c >= ' ' && c <= '~' && c != '\\' && c != '"') continue;
                put(d, s + from, (size_t)(i - from)); put_char(d, (Py_UCS4)(unsigned char)c); from = i + 1;
            }
            put(d, s + from, (size_t)(len - from));
        } else {
            int kind = PyUnicode_KIND(o);
            const void *data = PyUnicode_DATA(o);
            for (Py_ssize_t i = 0; i < len; i++) put_char(d, PyUnicode_READ(kind, data, i));
        }
        put(d, "\"", 1);
        return 0;
    }
    if ((PyTuple_CheckExact(o) || PyList_CheckExact(o)) && depth < 64) {
        put(d, "[", 1);
        for (Py_ssize_t i = 0; i < PySequence_Fast_GET_SIZE(o); i++) {
            if (i) put(d, ", ", 2);
            if (put_json(d, PySequence_Fast_GET_ITEM(o, i), depth + 1)) return -1;
        }
        put(d, "]", 1);
        return 0;
    }
    PyErr_SetString(PyExc_TypeError, "json_sha256_unsupported_value");
    return -1;
}

static PyObject *json_sha256(PyObject *Py_UNUSED(self), PyObject *obj) {
    Digest *d = PyMem_Malloc(sizeof *d);
    if (!d) return PyErr_NoMemory();
    CC_SHA256_Init(&d->ctx); d->n = 0;
    PyObject *result = NULL;
    if (!put_json(d, obj, 0)) {
        static const char hex[] = "0123456789abcdef";
        unsigned char md[CC_SHA256_DIGEST_LENGTH];
        char text[2 * CC_SHA256_DIGEST_LENGTH];
        CC_SHA256_Update(&d->ctx, d->buf, (CC_LONG)d->n);
        CC_SHA256_Final(md, &d->ctx);
        for (int i = 0; i < CC_SHA256_DIGEST_LENGTH; i++) { text[2 * i] = hex[md[i] >> 4]; text[2 * i + 1] = hex[md[i] & 0xf]; }
        result = PyUnicode_FromStringAndSize(text, sizeof text);
    }
    PyMem_Free(d);
    return result;
}

/* r3: the same json_sha256 bytes, fed incrementally: begin(); feed(state, seq, start, stop) appends
 * items seq[start:stop] of ONE top-level JSON array (the "[" and ", " separators exactly as above);
 * end(state) closes it with "]" and returns the hex digest. A failed feed poisons the state. */
typedef struct { Digest d; Py_ssize_t items; int state; } Stream;  /* state: 0 open, 1 failed, 2 ended */

static void stream_free(PyObject *capsule) { PyMem_Free(PyCapsule_GetPointer(capsule, "metawalk.json_sha256")); }

static PyObject *json_sha256_begin(PyObject *Py_UNUSED(self), PyObject *Py_UNUSED(unused)) {
    Stream *s = PyMem_Malloc(sizeof *s);
    if (!s) return PyErr_NoMemory();
    CC_SHA256_Init(&s->d.ctx); s->d.n = 0; s->items = 0; s->state = 0;
    PyObject *capsule = PyCapsule_New(s, "metawalk.json_sha256", stream_free);
    if (!capsule) PyMem_Free(s);
    return capsule;
}

static Stream *stream_arg(PyObject *capsule) {
    Stream *s = PyCapsule_IsValid(capsule, "metawalk.json_sha256") ? PyCapsule_GetPointer(capsule, "metawalk.json_sha256") : NULL;
    if (!s || s->state) { PyErr_SetString(PyExc_TypeError, "metawalk_arguments"); return NULL; }
    return s;
}

static PyObject *json_sha256_feed(PyObject *Py_UNUSED(self), PyObject *const *args, Py_ssize_t n) {
    Stream *s = n == 4 ? stream_arg(args[0]) : NULL;
    if (!s) { if (!PyErr_Occurred()) PyErr_SetString(PyExc_TypeError, "metawalk_arguments"); return NULL; }
    PyObject *seq = args[1];
    Py_ssize_t start = PyLong_Check(args[2]) ? PyLong_AsSsize_t(args[2]) : -1, stop = PyLong_Check(args[3]) ? PyLong_AsSsize_t(args[3]) : -1;
    if (PyErr_Occurred() || !(PyTuple_CheckExact(seq) || PyList_CheckExact(seq)) || start < 0 || stop < start) {
        if (!PyErr_Occurred()) PyErr_SetString(PyExc_TypeError, "metawalk_arguments");
        return NULL;
    }
    if (stop > PySequence_Fast_GET_SIZE(seq)) stop = PySequence_Fast_GET_SIZE(seq);
    for (Py_ssize_t i = start; i < stop; i++, s->items++) {
        put(&s->d, s->items ? ", " : "[", s->items ? 2 : 1);
        if (put_json(&s->d, PySequence_Fast_GET_ITEM(seq, i), 1)) { s->state = 1; return NULL; }
    }
    Py_RETURN_NONE;
}

static PyObject *json_sha256_end(PyObject *Py_UNUSED(self), PyObject *capsule) {
    Stream *s = stream_arg(capsule);
    if (!s) return NULL;
    if (!s->items) put(&s->d, "[", 1);
    put(&s->d, "]", 1);
    s->state = 2;
    static const char hex[] = "0123456789abcdef";
    unsigned char md[CC_SHA256_DIGEST_LENGTH];
    char text[2 * CC_SHA256_DIGEST_LENGTH];
    CC_SHA256_Update(&s->d.ctx, s->d.buf, (CC_LONG)s->d.n);
    CC_SHA256_Final(md, &s->d.ctx);
    for (int i = 0; i < CC_SHA256_DIGEST_LENGTH; i++) { text[2 * i] = hex[md[i] >> 4]; text[2 * i + 1] = hex[md[i] & 0xf]; }
    return PyUnicode_FromStringAndSize(text, sizeof text);
}

/* r3: live_ledger's pass-1 record step for consecutive rows whose stamp is an exact tuple, in row order:
 *   observations[path] = stamp; key = stamp[:2]; known = seen.get(key)
 *   new key -> seen[key] = [stamp, "image" if key == image_key else "disk" if key == disk_key else "scratch", [path]]
 *   known[0] != stamp -> raise error("ambiguous_inode_measurement:" + path); else known[2].append(path)
 *   progress("collect", len(observations)) whenever that length is a multiple of 4096 (progress not None).
 * record(rows, start, observations, seen, image_key, disk_key, error, progress) -> index of the first row
 * NOT handled here (any other stamp: pinned None, a fault, or an unexpected shape), or len(rows).
 * Pure dict/list work: no syscalls. */
static PyObject *record_rows(PyObject *Py_UNUSED(self), PyObject *const *args, Py_ssize_t n) {
    static PyObject *image, *disk, *scratch, *collect;
    if (!image && !(image = PyUnicode_InternFromString("image"))) return NULL;
    if (!disk && !(disk = PyUnicode_InternFromString("disk"))) return NULL;
    if (!scratch && !(scratch = PyUnicode_InternFromString("scratch"))) return NULL;
    if (!collect && !(collect = PyUnicode_InternFromString("collect"))) return NULL;
    if (n != 8 || !PyList_CheckExact(args[0]) || !PyLong_Check(args[1]) || !PyDict_CheckExact(args[2]) ||
        !PyDict_CheckExact(args[3]) || !PyType_Check(args[6]) || (args[7] != Py_None && !PyCallable_Check(args[7])))
        { PyErr_SetString(PyExc_TypeError, "metawalk_arguments"); return NULL; }
    PyObject *rows = args[0], *observations = args[2], *seen = args[3], *progress = args[7];
    Py_ssize_t i = PyLong_AsSsize_t(args[1]);
    if (i < 0) { if (!PyErr_Occurred()) PyErr_SetString(PyExc_TypeError, "metawalk_arguments"); return NULL; }
    for (; i < PyList_GET_SIZE(rows); i++) {
        PyObject *row = PyList_GET_ITEM(rows, i);
        if (!PyTuple_CheckExact(row) || PyTuple_GET_SIZE(row) != 2) break;
        PyObject *path = PyTuple_GET_ITEM(row, 0), *stamp = PyTuple_GET_ITEM(row, 1);
        if (!PyTuple_CheckExact(stamp) || !PyUnicode_CheckExact(path)) break;
        Py_INCREF(row);  /* keeps path/stamp alive across the calls below */
        int r = PyDict_SetItem(observations, path, stamp);
        PyObject *key = r ? NULL : PyTuple_GetSlice(stamp, 0, 2);
        PyObject *known = key ? PyDict_GetItemWithError(seen, key) : NULL;  /* borrowed */
        if (!key || (!known && PyErr_Occurred())) r = -1;
        else if (!known) {
            int is_image = PyObject_RichCompareBool(key, args[4], Py_EQ);
            int is_disk = is_image == 0 ? PyObject_RichCompareBool(key, args[5], Py_EQ) : 0;
            PyObject *paths = (is_image < 0 || is_disk < 0) ? NULL : PyList_New(1);
            PyObject *value = paths ? PyList_New(3) : NULL;
            if (value) {
                PyList_SET_ITEM(paths, 0, Py_NewRef(path));
                PyList_SET_ITEM(value, 0, Py_NewRef(stamp));
                PyList_SET_ITEM(value, 1, Py_NewRef(is_image ? image : is_disk ? disk : scratch));
                PyList_SET_ITEM(value, 2, paths); paths = NULL;
                r = PyDict_SetItem(seen, key, value);
            } else r = -1;
            Py_XDECREF(paths); Py_XDECREF(value);
        } else {
            PyObject *old = PyList_Check(known) && PyList_GET_SIZE(known) == 3 ? PyList_GET_ITEM(known, 0) : NULL;
            PyObject *list = old ? PyList_GET_ITEM(known, 2) : NULL;
            if (!old || !PyList_Check(list)) {  /* not the [stamp, category, paths] shape: leave the row to
                                                * Python, whose identical observations[path] = stamp is a no-op */
                Py_DECREF(key); Py_DECREF(row);
                break;
            }
            int differ = PyObject_RichCompareBool(old, stamp, Py_NE);
            if (differ > 0) r = fail(args[6], "ambiguous_inode_measurement:", path);
            else r = differ < 0 ? -1 : PyList_Append(list, path);
        }
        Py_XDECREF(key);
        Py_DECREF(row);
        if (r) return NULL;
        if (progress != Py_None && !(PyDict_GET_SIZE(observations) & 4095)) {
            PyObject *done = PyObject_CallFunction(progress, "On", collect, PyDict_GET_SIZE(observations));
            if (!done) return NULL;
            Py_DECREF(done);
        }
    }
    return PyLong_FromSsize_t(i);
}

/* r5: 1 when every item of `paths` is in `known` (frozenset.issuperset(paths) for a tuple of str), 0 if
 * not, -1 on error. Same containment calls, in order, stopping at the first missing item. */
static int all_known(PyObject *known, PyObject *paths) {
    for (Py_ssize_t i = 0; i < PyTuple_GET_SIZE(paths); i++) {
        int r = PySet_Contains(known, PyTuple_GET_ITEM(paths, i));
        if (r <= 0) return r;
    }
    return 1;
}

/* r5: live_ledger._snapshot_parts' entry step for items[start:stop] of sorted(seen.items()), in order:
 *   e = _entry(key, value)    # Entry.__new__(Entry); e.__dict__.update(device=key[0], inode=key[1],
 *                             #   allocated=value[0][5] * 512, size=value[0][4], category=value[1],
 *                             #   paths=tuple(value[2]) if len(value[2]) == 1 else tuple(sorted(value[2])))
 *   entries.append(e)
 *   if not known.issuperset(e.paths): allocations.append((e.device, e.inode, e.allocated, e.size, e.category))
 * parts_entries(items, start, stop, Entry, known, entries, allocations) -> index of the first item NOT handled
 * (any item outside the exact (tuple key, [tuple stamp, category, list paths]) shape), or min(stop, len(items));
 * live_ledger performs exactly the Python step for that item. Pure object work: no syscalls. */
static PyObject *parts_entries(PyObject *Py_UNUSED(self), PyObject *const *args, Py_ssize_t n) {
    static PyObject *k_device, *k_inode, *k_allocated, *k_size, *k_category, *k_paths, *blocksize, *noargs;
    if (!k_device && !(k_device = PyUnicode_InternFromString("device"))) return NULL;
    if (!k_inode && !(k_inode = PyUnicode_InternFromString("inode"))) return NULL;
    if (!k_allocated && !(k_allocated = PyUnicode_InternFromString("allocated"))) return NULL;
    if (!k_size && !(k_size = PyUnicode_InternFromString("size"))) return NULL;
    if (!k_category && !(k_category = PyUnicode_InternFromString("category"))) return NULL;
    if (!k_paths && !(k_paths = PyUnicode_InternFromString("paths"))) return NULL;
    if (!blocksize && !(blocksize = PyLong_FromLong(512))) return NULL;
    if (!noargs && !(noargs = PyTuple_New(0))) return NULL;
    if (n != 7 || !PyList_CheckExact(args[0]) || !PyLong_Check(args[1]) || !PyLong_Check(args[2]) || !PyType_Check(args[3]) ||
        !PyFrozenSet_CheckExact(args[4]) || !PyList_CheckExact(args[5]) || !PyList_CheckExact(args[6]))
        { PyErr_SetString(PyExc_TypeError, "metawalk_arguments"); return NULL; }
    PyObject *items = args[0], *known = args[4], *entries = args[5], *allocations = args[6];
    PyTypeObject *type = (PyTypeObject *)args[3];
    Py_ssize_t i = PyLong_AsSsize_t(args[1]), stop = PyLong_AsSsize_t(args[2]);
    if (i < 0 || stop < 0) { if (!PyErr_Occurred()) PyErr_SetString(PyExc_TypeError, "metawalk_arguments"); return NULL; }
    for (; i < stop && i < PyList_GET_SIZE(items); i++) {
        PyObject *item = PyList_GET_ITEM(items, i);
        if (!PyTuple_CheckExact(item) || PyTuple_GET_SIZE(item) != 2) break;
        PyObject *key = PyTuple_GET_ITEM(item, 0), *value = PyTuple_GET_ITEM(item, 1);
        if (!PyTuple_CheckExact(key) || PyTuple_GET_SIZE(key) < 2 || !PyList_CheckExact(value) || PyList_GET_SIZE(value) < 3) break;
        PyObject *stamp = PyList_GET_ITEM(value, 0), *category = PyList_GET_ITEM(value, 1), *list = PyList_GET_ITEM(value, 2);
        if (!PyTuple_CheckExact(stamp) || PyTuple_GET_SIZE(stamp) < 6 || !PyList_CheckExact(list)) break;
        Py_INCREF(item);  /* keeps key/value/stamp/category/list alive across the calls below */
        PyObject *entry = NULL, *dict = NULL, *allocated = NULL, *paths = NULL, *row = NULL;
        int r = -1;
        if (PyList_GET_SIZE(list) == 1) paths = PyList_AsTuple(list);
        else {
            PyObject *copy = PyList_GetSlice(list, 0, PyList_GET_SIZE(list));  /* sorted(value[2]) */
            if (copy && !PyList_Sort(copy)) paths = PyList_AsTuple(copy);
            Py_XDECREF(copy);
        }
        if (paths && (allocated = PyNumber_Multiply(PyTuple_GET_ITEM(stamp, 5), blocksize)) &&
            (entry = type->tp_new(type, noargs, NULL)) && (dict = PyObject_GenericGetDict(entry, NULL)) &&
            !PyDict_SetItem(dict, k_device, PyTuple_GET_ITEM(key, 0)) && !PyDict_SetItem(dict, k_inode, PyTuple_GET_ITEM(key, 1)) &&
            !PyDict_SetItem(dict, k_allocated, allocated) && !PyDict_SetItem(dict, k_size, PyTuple_GET_ITEM(stamp, 4)) &&
            !PyDict_SetItem(dict, k_category, category) && !PyDict_SetItem(dict, k_paths, paths) &&
            !PyList_Append(entries, entry)) {
            int superset = all_known(known, paths);
            if (superset == 1) r = 0;
            else if (superset == 0 && (row = PyTuple_Pack(5, PyTuple_GET_ITEM(key, 0), PyTuple_GET_ITEM(key, 1), allocated,
                                                          PyTuple_GET_ITEM(stamp, 4), category)))
                r = PyList_Append(allocations, row);
        }
        Py_XDECREF(row); Py_XDECREF(dict); Py_XDECREF(entry); Py_XDECREF(allocated); Py_XDECREF(paths);
        Py_DECREF(item);
        if (r) return NULL;
    }
    return PyLong_FromSsize_t(i);
}

/* r5: live_ledger._snapshot_parts' metadata step for keys[start:stop] of sorted(observations), in order:
 *   if path not in known: metadata.append(("metadata", path) + observations[path])
 * parts_metadata(keys, start, stop, observations, known, tag, metadata) -> index of the first key NOT handled
 * (not an exact str, or its observation not an exact tuple), or min(stop, len(keys)). No syscalls. */
static PyObject *parts_metadata(PyObject *Py_UNUSED(self), PyObject *const *args, Py_ssize_t n) {
    if (n != 7 || !PyList_CheckExact(args[0]) || !PyLong_Check(args[1]) || !PyLong_Check(args[2]) || !PyDict_CheckExact(args[3]) ||
        !PyFrozenSet_CheckExact(args[4]) || !PyUnicode_CheckExact(args[5]) || !PyList_CheckExact(args[6]))
        { PyErr_SetString(PyExc_TypeError, "metawalk_arguments"); return NULL; }
    PyObject *keys = args[0], *observations = args[3], *known = args[4], *tag = args[5], *metadata = args[6];
    Py_ssize_t i = PyLong_AsSsize_t(args[1]), stop = PyLong_AsSsize_t(args[2]);
    if (i < 0 || stop < 0) { if (!PyErr_Occurred()) PyErr_SetString(PyExc_TypeError, "metawalk_arguments"); return NULL; }
    for (; i < stop && i < PyList_GET_SIZE(keys); i++) {
        PyObject *path = PyList_GET_ITEM(keys, i);
        if (!PyUnicode_CheckExact(path)) break;
        int skip = PySet_Contains(known, path);
        if (skip < 0) return NULL;
        if (skip) continue;
        PyObject *stamp = PyDict_GetItemWithError(observations, path);  /* borrowed */
        if (!stamp) { if (PyErr_Occurred()) return NULL; break; }
        if (!PyTuple_CheckExact(stamp)) break;
        Py_ssize_t size = PyTuple_GET_SIZE(stamp);
        PyObject *row = PyTuple_New(size + 2);
        if (!row) return NULL;
        PyTuple_SET_ITEM(row, 0, Py_NewRef(tag));
        PyTuple_SET_ITEM(row, 1, Py_NewRef(path));
        for (Py_ssize_t k = 0; k < size; k++) PyTuple_SET_ITEM(row, k + 2, Py_NewRef(PyTuple_GET_ITEM(stamp, k)));
        int r = PyList_Append(metadata, row);
        Py_DECREF(row);
        if (r) return NULL;
    }
    return PyLong_FromSsize_t(i);
}

static PyMethodDef methods[] = {
    {"parts_entries", (PyCFunction)(void (*)(void))parts_entries, METH_FASTCALL, "Snapshot entry/allocation step (live_ledger order)."},
    {"parts_metadata", (PyCFunction)(void (*)(void))parts_metadata, METH_FASTCALL, "Snapshot metadata step (live_ledger order)."},
    {"record", (PyCFunction)(void (*)(void))record_rows, METH_FASTCALL, "Pass-1 record step for ordinary rows (live_ledger order)."},
    {"json_sha256_begin", json_sha256_begin, METH_NOARGS, "Incremental json_sha256 of one top-level array: new state."},
    {"json_sha256_feed", (PyCFunction)(void (*)(void))json_sha256_feed, METH_FASTCALL, "Append seq[start:stop] items."},
    {"json_sha256_end", json_sha256_end, METH_O, "Close the array; hex digest."},
    {"scan", (PyCFunction)(void (*)(void))scan, METH_FASTCALL, "No-follow pre-order metadata walk over all roots (pass 1)."},
    {"restat", (PyCFunction)(void (*)(void))restat, METH_FASTCALL, "Fresh no-follow lstat pass (pass 2)."},
    {"json_sha256", json_sha256, METH_O, "sha256 hex of json.dumps(obj, sort_keys=True).encode(), streamed."},
    {NULL, NULL, 0, NULL},
};

static struct PyModuleDef module = {PyModuleDef_HEAD_INIT, .m_name = "_metawalk", .m_size = 0, .m_methods = methods};

PyMODINIT_FUNC PyInit__metawalk(void) { return PyModule_Create(&module); }
