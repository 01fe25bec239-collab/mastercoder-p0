# Corpus Graph Summary

## Construction

The complete extracted Markdown corpus was indexed as a persistent evidence graph. Deterministic document nodes and explicit filename/reference relationships were deduplicated before clustering.

- Documents/nodes: 296
- Edges after deduplication: 566
- Communities: 133
- Health: no dangling targets, missing source locations, self-loops, or collapsed duplicate endpoints
- Benchmark-reported average query reduction: approximately 3.2x against its displayed naive corpus estimate
- Graphify API token tracker: 0 input / 0 output, because extraction was deterministic and host-assisted rather than dispatched through Graphify's metered semantic backend; this is not a claim that the surrounding synthesis used zero model effort

## Auditable query expansion

The synthesis query used only graph-vocabulary tokens:

architecture, approval, publication, merge, harness, attempt, binding, sandbox, budget, eligibility, evidence, version.

The breadth-first traversal returned 90 relevant nodes. Its highest-value roots included both patch/publication designs, agent harness/graph, Git-object/merge audit, FullAttemptBinding audit, approval UI audit, budget/eligibility audits, sandbox audit, and protocol-version-skew audit.

## Synthesis consequence

The graph independently reinforced eight cross-corpus root seams:

1. exact candidate/current provider check
2. structured harness with credential confinement
3. noncircular canonical attempt identity
4. sandbox capability/no downgrade
5. evidence provenance versus independence/truth
6. root resource budget and late settlement
7. separate human approval and confined publication
8. protocol/generation/currentness across authorities

The graph is an orientation and traceability aid. Final decisions were verified against full documents and audit finding text, not inferred from graph topology alone.
