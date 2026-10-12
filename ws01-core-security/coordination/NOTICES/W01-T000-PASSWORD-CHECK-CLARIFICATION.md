# W01-T000 password grep criterion — proposed clarification

PROTOTYPE — DISPOSABLE — NOT PRODUCTION

STATUS: OWNER_APPROVED_IN_CHAT, 2026-10-13 IST. Owner reply: “Approve the clarification”. Recorded by A1; no agent signature on behalf of master.
TASK: W01-T000-WORKSPACE-SCAFFOLD r0
BASE_SHA: 58b6ab38b0a514d94d8851679ad66ef9bf696c9e
IMPLEMENTATION_SHA: 30f4813838fa779ec55af45cb2892f417e4205b4
TREE: cc6722e79d5219c3f804ba4d75be1f9e7b176442

The literal negative check says recursive `git grep -n "POSTGRES_PASSWORD=" ws01-core-security` finds only the product `.env.example` placeholder. At both the base and candidate, it finds the identical three lines: product `.env.example:60` placeholder, coordination `.env.example:60` placeholder, and coordination `task.md:127` quoting the check itself. Neither coordination file changed. This baseline contradiction cannot be repaired within the product task scope.

Proposed evaluation, retaining the original task text and full command output:

1. Run the original recursive grep and record all its actual matches; never report a literal sole-hit PASS.
2. Require the two coordination matches/files to remain byte-identical to the protected base. Any changed/new coordination match or forbidden coordination edit is a failure, not automatically excluded.
3. In the tracked product subtree, with only the unchanged coordination subtree separated for classification, require exactly one `POSTGRES_PASSWORD=` hit: `ws01-core-security/.env.example` containing `POSTGRES_PASSWORD=__SET_ME__`. A new match in source, compose, docs or another product file fails. Compose must independently consume the environment value without a literal password.
4. Retain staged/attempt secret scans and the actual full-history scan result. The known historical contract-name findings are not reported as clean or blanket-suppressed. A3 independently checks this evidence and all other criteria.

This narrows a contradictory search expectation to its stated product-secret objective while adding exact checks for unchanged baseline matches. It does not waive secret detection, remove files, expand A2 scope, authorize PR/merge, or record final A3 PASS. The original specification is preserved verbatim.

Authority requiring a decision: coordination/AGENTS.md §4 protocol DISPATCH says “if a field looks wrong, A1 asks the human (QUESTION FOR USER) instead of editing it.” A1 has verified the baseline/candidate evidence; owner approval is requested for this concrete evaluation rule.
