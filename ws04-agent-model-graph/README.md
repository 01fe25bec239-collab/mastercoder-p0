# PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)

# ws04-agent-model-graph

Agent, model and graph workstream: model broker and secret dialer, adapter, budget ledger, graph engine, dependency acquisition, and the
harnesses for P0-01A/B/C/D, P0-08B/C/D and P0-05E/F/G. **PROTOTYPE — DISPOSABLE — NOT PRODUCTION.** Production implementation is not authorized.

Owner and scope: coordination/SCOPE.md. Write set: coordination/WRITE_SET.md. Agent rules: coordination/AGENTS.md + coordination/SKILLS.md.

## Layout

| Path | What |
|---|---|
| crates/ | 13 library crates (the secret dialer also has a binary) |
| doubles/ | `fake-model-provider/v1` and `model-broker/v1` (test doubles; never evidence) |
| claims/ | 10 harness binaries, one per claim (see claims/README.md) |
| db/ | migration modules (`db/MODULES`): `model`, `graph`, `dependency_grants`, `dependency_cache` |
| docs/ | MODULES.md: ownership, imports and the dependency rule |

The workspace path-depends on `../ws01-core-security` and `../ws03-runner-sandbox-acquisition`; it has no `rust-toolchain.toml` of its own
(the repository root pin is the single toolchain source).

## Configuration and local checks

Configuration comes from `.env.example` -> `.env.local` (symlink to the lane `.env`); nothing is hardcoded. The local test database is the
`postgres` service in `compose.yaml` on `MC_WS04_PG_PORT`. Start it from the COORDINATION worktree only; task worktrees never run compose up/down.

```
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features --locked -- -D warnings
cargo test --workspace --locked
```

No model API key exists in CI, and no real provider is called during BUILD: the live driver runs only in W04-RUN-P0-01A after G2 + G3.
