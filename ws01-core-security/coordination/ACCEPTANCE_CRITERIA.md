# ACCEPTANCE CRITERIA — WORKSTREAM-01-CORE-SECURITY-AUTHORITY (freeze-closure block)

The workstream is DONE for this block when all of the following hold (DEFINITION_OF_DONE.md defines task/claim done-ness):

1. Every W01 BUILD task INTEGRATED (A3 PASS on exact SHA + merged with merge commit).
2. Interfaces I-01, I-02, I-03, I-04, I-05, I-07, I-08, I-09, I-16 published at @v1 with doubles; no breaking change without vN+1.
3. All 13 W01 claims either ACCEPTED (two-lens evidence) with an honest outcome, or FAIL/INCONCLUSIVE with cell disabled + CCR/ADR note.
4. External audit result recorded; every finding converted to a CCR or a reasoned no-change; G2 decided.
5. No open W01 CCR; memory.md and latest A1 handoff current.
6. Every gap assigned to lane 01 (W01) in `COMMON/UNASSIGNED_GAP_AUDIT.md` is closed by its task.
7. No secret ever committed (gitleaks history scan clean); every file carries the PROTOTYPE header.
8. The W06 freeze dossier shows this workstream's claims and cells with no unexplained state.
