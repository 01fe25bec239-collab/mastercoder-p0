# W06 root toolchain pin — all-lane premerge notice

PROTOTYPE — DISPOSABLE — NOT PRODUCTION

FROM: STARTUP-A1 (acting W06-A1)
TO: W01/W02/W03/W04/W05/W06 A1
MESSAGE_TYPE: INTERFACE_NOTICE
DATE_UTC: 2026-10-13T00:26:25.187301+00:00
TASK_ID: W06-T000-ROOT-HARDENING
CANDIDATE: b780a3f4fa4f550b76a79bc9483c1f94cd7f2d03
CANONICAL_PR: https://github.com/01fe25bec239-collab/mastercoder-p0/pull/1
TOOLCHAIN: Rust1.97.1 exact; componentsclippy/rustfmt, minimalprofile
CANDIDATE_FILE_SHA256: 780a002a948bbe427a267789d8a5f3e36fa838d596b8b240985f17dc1cd71360

The candidate retains the import's Rust1.97.1 version; this is not a toolchain upgrade. Root comments/minimalprofile hardening is part of W06's reviewedrootcommit. AllRustworkspaces resolve the rootpin and must use lockedcommands/sharedsinglebuildslot under existingcaps. Rebuild/CI evidence belongs to each owningtask; this notice does not claim allsixworkspacesbuild or qualifyruntime. No otherlaneproduct/manifests/locks edited and no install/upgrade authorized by this notice.

This fulfills the T000A1 premerge notificationrecord, not acceptance or merge. W06commissioningA3pending; owner alreadyauthorizedconditionalnormalmerge and stagedcanonicalmainfinalacceptance. After any merge obtain actualmainnewguardgreen and freshfinalA3beforeINTEGRATED/dependencyunlock. AllclaimsNOT_RUN/G2NOT_PASSED/G3none/productionunauthorized.
