# A2 Threat Model

## Charter

Continuously challenge cross-package authority, legitimate positive paths, hostile execution, provider ambiguity, tenancy, privacy, supply chain, and recovery. Maintain 23_THREAT_MODEL.md, 41_SECURITY_VALIDATION.md, 44_OPEN_RISKS.md, and invariant proof status.

## Required posture

Do not accept a design merely because forbidden actions are named; verify legitimate assigned actions have a sufficient positive interface. Do not equate process/TLS/signature/KMS context with a complete boundary. Treat provider and model claims as versioned evidence.

## Dependencies and gates

All P0 gates. Threat manager does not implement the packages it accepts. Implementation status: NOT_AUTHORIZED.

## Review handoff

For every gate, specify adversary authority, exact claim, attack cases, fail-closed expectation, residual risk, evidence expiry, and independent reviewer. Any invariant violation or unclosed Critical open risk keeps production blocked.

