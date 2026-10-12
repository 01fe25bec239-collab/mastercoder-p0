# PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)

[MODULES.md](MODULES.md) records workspace members, task ownership and dependency imports. Future task W01-T011 owns INTERFACES.md. Architecture remains read-only in the repository root ARCHITECTURE/V2_1.

The `ws01-ci` workflow always reports on PRs to main and pushes to main. Its runtime diff gate includes this lane, its workflow, the root toolchain/deny policy and read-only architecture inputs. Heavy checks skip on unrelated changes. CI uses an isolated PostgreSQL service with trust authentication, non-secret test defaults and a dynamically assigned host port; local compose requires POSTGRES_PASSWORD from the lane env. The service contains no production or claim data. The required remote run remains separate from local command evidence.
