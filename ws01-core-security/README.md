# PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)

# W01 core security workspace

This empty scaffold contains 21 crates, six Rust doubles, 13 claim harnesses and the codegen tool. It implements no security behavior or SQL tables. All claims remain NOT_RUN; production implementation is NOT_AUTHORIZED.

See [MODULES](docs/MODULES.md) for task ownership, imports and exact dependency pins, [docs](docs/README.md), [doubles](doubles/README.md) and [claims](claims/README.md). Agent rules remain in coordination/AGENTS.md and coordination/SKILLS.md; use the live coordinator records named in your dispatch.

Use the repository root Rust toolchain pin. Run Cargo from this directory with `--locked` after initial lock generation: `cargo build --workspace --locked`, `cargo fmt --all --check`, `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings`, `cargo test --workspace --locked`. Local dependency resolution and checks must use the dispatch's shared single-build wrapper with this directory as `--cwd`; load `.env.local` explicitly without printing it.

Compose requires the lane's untracked env values listed in `.env.example`. Validate without displaying the resolved password with `docker compose --env-file .env.local config --quiet`; `config --no-interpolate` inspects placeholders. Database startup remains deferred to A1 after integration. A2 never runs compose up/down.

Migration module order is in `db/authority/MODULES`. Future SQL files use `db/authority/<module>/NNNN_<name>.sql`, with numbering local to each module. One PostgreSQL database contains the future `mc_authority` schema. No migrations exist in this task.
