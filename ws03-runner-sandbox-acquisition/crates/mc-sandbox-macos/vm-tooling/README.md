# macOS VM tooling prototype

PROTOTYPE — DISPOSABLE — NOT PRODUCTION. This is a source-only review package, not a runnable installation or Rust backend integration. It selects 26 source files from the frozen 41-member R15 manifest (`567408501bc6c6b07ebb58ae375b0f68e78e254fb2e4af4f4d4b96c82ec611da`). The heartbeat scheduling change is confined to `source/native_control.py`; the other 25 packaged source files remain byte-identical to the prior source package. `provenance.manifest.json` binds the selected source and tests.

`tests/test_heartbeat.py` is copied unchanged from the R15 controls and covers heartbeat cadence, immediate handoff, stop conditions and send failure. The retained `tests/test_r12_minimum.py` and fixture are unchanged. That focused test needs a compatible macOS CPython 3.14 `_metawalk` extension, which is not included, and is not portable from this source-only package.

Compiled binaries, host configuration, generated digest expectations, runtime state and VM assets are excluded. The native controller pins a larger complete runtime set than this package contains, so this directory grants no launch authority. Current native validation remains pending; no operational readiness is claimed.
