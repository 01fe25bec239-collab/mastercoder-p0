# W01-T000 r1 remote CI proposal

PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)

STATUS: PROPOSED_NOT_AUTHORIZED
DATE_UTC: 2026-10-13T05:13:40.784245+00:00
OWNER: HUMAN:master (program owner)

The independent preliminary A3 found the complete scaffold locally satisfactory and closed F-1/F-2 locally. The immutable candidate is d926be555e86ad2f382fa2d3f73ab6d6998c8e41, tree e3024c237e6fdf3762a1bd874b8e67732f4a3012, branch ws01/w01-t000-workspace-scaffold-r1, parent 30f4813838fa779ec55af45cb2892f417e4205b4, original protected base 58b6ab38b0a514d94d8851679ad66ef9bf696c9e. Original task fields and acceptance criteria remain unchanged. Review SHA-256: e2d1090337436e31b5838ac50837f0fb0218cf6fe290d19adc7a98ac9df5dded.

## Concrete remote scope for approval

1. Push only the existing signed candidate branch to private repository https://github.com/01fe25bec239-collab/mastercoder-p0. Open a PR to main titled `[W01-T000-WORKSPACE-SCAFFOLD] Scaffold the core security workspace`. HEAD must remain d926be555e86ad2f382fa2d3f73ab6d6998c8e41. Current canonical main is 5f92752cac8d9ba39ef20a6caf4c1c95227d7036; its newer W06 guards must remain in the eventual normal merge. Do not rebase, amend, add a main merge to this returned branch, or substitute another reviewed head.
2. Collect all eight actual required Actions app15368 contexts: arch-check, write-set-guard, ws01-ci, ws02-ci, ws03-ci, ws04-ci, ws05-ci and ws06-ci. Bind check/run/job/log evidence to the exact PR head and actual GitHub merge-test subject where applicable. Verify the W01 heavy steps really run, including service startup, locked formatting, build/Clippy/tests and specification checks. Skipped heavy steps on unchanged other lanes are accounted for explicitly; a skipped/missing required job is blocking.
3. Create one isolated signed synthetic formatting-negative commit stacked on d926be555e86ad2f382fa2d3f73ab6d6998c8e41, changing only `ws01-core-security/crates/mc-contracts/src/lib.rs` by appending deliberately misformatted but valid Rust `pub fn w01_format_control( ) { }
`. Push only the synthetic branch `ws01/w01-t000-workspace-scaffold-r9001` and open one PR to main with title `[W01-T000-WORKSPACE-SCAFFOLD] CI negative formatting`. The r9001 name is a guard-recognized disposable fixture identifier, not a repair attempt or accepted implementation. Check it is unused first; a collision stops this step. Preserve the original implementation/reviews byte-for-byte. Require an actual formatting-step failure with the expected formatter diff, with scope/architecture guards green; infrastructure or unrelated failures do not prove the negative criterion. Do not merge this fixture. Retain raw head/tree/parent/delta/check/job/step/log evidence, then close this one test PR and delete only its enumerated synthetic branch. Keep its commit and retained evidence locally.
4. Babysit the candidate PR for meaningful CI, comments and review-thread changes. No A1 product fixes: a real new defect requires a bounded fresh A2 and fresh A3. A changed candidate invalidates the earlier binding. CodeRabbit quota messages are not reviews or approval; do not enable paid review.
5. After genuine positive and formatting-negative evidence, arrange a separate fresh independent final exact-SHA/tree A3 using the retained evidence plus live validation. No ACCEPTED_SHA, final PASS, dependency unlock or integration before that report meets every criterion. Only A1 processes the independent result.

Cost: included GitHub Actions only, $0 additional spend; first inspect availability without changing billing settings. No paid review/model API/cloud/keys or budget increase. No new repository, provider App, secret/environment, protection change, claim or production action. Preserve eight required contexts, admin enforcement, current strict:false and merge-only settings.

## Later merge

This proposal requests push/PR/testing/babysitting and the bounded test cleanup. It does not request or grant canonical merge yet. If the owner also explicitly authorizes a conditional merge, use only normal merge with an exact-head guard after final A3 and all real CI, fresh all-page comments/reviews/threads/protection/base/head/mergeability checks. No force/rebase/squash/bypass/auto-merge or canonical branch deletion. Otherwise obtain a separate concrete merge decision after acceptance. Verify actual merged provenance and main checks before recording INTEGRATED.

Prior owner W06 remote/merge commissioning approval applies only to W06 PR1. The general BUILD_REVIEW_CI_SEQUENCE_DECISION approves ordering, not outward actions. AGENTS.md §14 requires explicit owner authorization for this W01 push and PR scope.
