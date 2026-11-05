# ws02-git-analysis-review

# PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)

Rust workspace for the Git, analysis and review lane (W02): 12 crates, 3 test doubles, 12 claim harnesses, `db/git` migration modules, compose and CI. This is an empty compiling scaffold (W02-T000); later tasks only fill in their own module files.

- Layout and owning task per module: [docs/MODULES.md](docs/MODULES.md); docs index: [docs/README.md](docs/README.md).
- Owner and scope: coordination/SCOPE.md. Write set: coordination/WRITE_SET.md. Agent rules: coordination/AGENTS.md + coordination/SKILLS.md.
- Build/test from this directory with `cargo build --workspace` / `cargo test --workspace --locked`. The toolchain comes from the repository-root `rust-toolchain.toml` (no workspace-level pin).
- Configuration comes from the lane `.env` (see `.env.example`); no secrets or hardcoded values in source.
