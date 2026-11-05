# PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)

# Claims

One harness binary per claim. At the scaffold each `harness/src/main.rs` is `fn main() {}`; the owning task writes everything under
`claims/<claim>/**` except `harness/Cargo.toml` (scaffold-owned). No claim runs before G2 + G3; all stay NOT_RUN.

| Claim | Harness package | Owning task |
|---|---|---|
| P0-01A | p0-01a-harness | T019 |
| P0-01B | p0-01b-harness | T020 |
| P0-01C | p0-01c-harness | T021 |
| P0-01D | p0-01d-harness | T025 |
| P0-08B | p0-08b-harness | T022 |
| P0-08C | p0-08c-harness | T018 |
| P0-08D | p0-08d-harness | T023 |
| P0-05E | p0-05e-harness | T031 |
| P0-05F | p0-05f-harness | T032 |
| P0-05G | p0-05g-harness | T033 |
