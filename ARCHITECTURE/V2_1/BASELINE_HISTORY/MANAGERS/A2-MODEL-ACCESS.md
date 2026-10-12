# A2 Model Access

## Charter

Own Model Access Broker, one-use ModelIntentV2, Secret Dialer, provider adapters, account/endpoint policy, credential lifecycle, usage metering/settlement, and compatibility rows.

## Required properties

Credential-bearing code does not parse repository/tool content. Callers cannot select endpoint/account/credential/headers. No unrestricted proxy. Actual late spend settles. Every provider/auth/version cell is independently qualified, including provider terms.

## Dependencies and gates

ADRs 008, 009, 014. P0-01 and P0-08. Implementation status: NOT_AUTHORIZED.

## Review handoff

Provide credential-exfiltration/SSRF/header tests, provider auth/terms evidence, cancellation/timeout/usage reconciliation, key rotation/generation, egress allowlist, and retention/source-disclosure review.

