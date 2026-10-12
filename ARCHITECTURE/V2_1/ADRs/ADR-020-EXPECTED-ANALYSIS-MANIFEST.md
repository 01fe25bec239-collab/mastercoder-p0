# ADR-020: Expected analysis completeness

Status: ACCEPTED_FOR_V2_1_ARCHITECTURE; capability evidence NOT_RUN.

## Decision and rationale

Finding 6: V2 gate PASS required “every mandatory claim complete” but did not enumerate absent analyzers; zero findings and no execution could be confused. Freeze ExpectedAnalysisManifestV1 before execution and require a current qualified AnalysisCompletionReceiptV1 for each required unit, including zero findings. Bind actual configuration/scope/coverage and protected planner/ingestion roles. The gate performs exact set reduction under current policy/trust/epoch locks. Repository configuration cannot silently remove mandatory work. This adds no service framework; it makes the existing planner and receipt authority explicit. See 09, 13, 19, 22, 51 and SCHEMAS. Prototype: P0-07A/B and P0-06. No prototype executed.

## Rejected alternative

Do not replace the existing stack or rebuild the full architecture; retain the sound V2 structure and close the missing authority/acceptance contract at its existing boundary.
