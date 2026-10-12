# Exact Git Lineage

## Coordinates

Git identity is algorithm qualified. V2 records object format/algorithm and raw OID bytes, not ambiguous strings. The canonical supported production cell begins with SHA-1 GitHub repositories only if P0-03 passes, while the data contract permits future algorithms without reinterpreting old identifiers.

MergeSubjectV2 includes:

- provider and stable repository ID
- repository_tenant_epoch and PR number
- head commit and tree
- base ref and observed base commit/tree
- candidate kind: HEAD_ONLY_NONMERGE, TEST_MERGE, MERGE_GROUP, PROVIDER_EQUIVALENT
- exact candidate commit and tree
- intended merge method and provider generation/event
- observation time/expiry and protocol version

## Gate subject

PASS applies only to the exact candidate tree verified. A head-only run may be advisory but cannot authorize a base-integrated merge when integration behavior is material. Merge queue evaluations bind the temporary merge_group SHA/tree. Normal, merge, squash, and rebase modes require a staging-proven provider mapping or remain GATE_UNVERIFIABLE.

## Freshness

Head/base/candidate OID, queue generation, repository epoch, policy digest, capability digest, authority_epoch, or evaluation sequence change makes the previous identity stale. Provider branch settings are not trusted to preserve base freshness; the Review Broker reads current state and invalidates CheckProjectionV2 under 54, including already successful checks.

## Check semantics

GitHub required checks may consider success, neutral, or skipped acceptable in some configurations, so V2 never uses neutral/skipped/success for mandatory incomplete work. The broker owns the conclusion mapping. P0-03 proves exact repository/ruleset/queue behavior, duplicate/shared-SHA selection, App pinning, delayed writes and freeze effectiveness before enablement. Internal stale/revoked state triggers provider withdrawal and reconciliation; the interval before effective withdrawal/freeze can still permit a merge and is explicitly EXTERNAL_GATE_UNSAFE (54).

## Publication lineage

Patch base commit/tree, candidate tree, patch/manifest digest, verifier environment, approval proof, created tree/commit/ref, and fix PR are linked immutably. Pinned PatchCustodyV1 bytes feed independent verification, AA-controlled complete rendering and gateway materialization. Returned-tree equality verifies exact candidate reconstruction, while plan/custody/display digests bind the actual approved patch and effects. Gateway allows only the exact new fix ref; underlying token permissions have the residual risk in 53.

## Official references

- [GitHub required status checks](https://docs.github.com/en/pull-requests/how-tos/merge-and-close-pull-requests/troubleshooting-required-status-checks)
- [GitHub merge queues](https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/configuring-pull-request-merges/managing-a-merge-queue)
- [Git database trees](https://docs.github.com/en/rest/git/trees)
- [GitHub App installation tokens](https://docs.github.com/en/apps/creating-github-apps/authenticating-with-a-github-app/generating-an-installation-access-token-for-a-github-app)

