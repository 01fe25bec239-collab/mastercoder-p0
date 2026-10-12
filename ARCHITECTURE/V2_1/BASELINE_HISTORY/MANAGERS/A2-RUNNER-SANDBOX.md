# A2 Runner and Sandbox

## Charter

Own Rust runner enrollment/session, capability advertisements, lease protocol, workspace materialization, S0-S3 sandbox backends, resource/egress control, reset, receipts, cancellation, and cross-platform cells.

## Required properties

Exact capability match; no downgrade; no reusable secrets in guests; S2 VM-grade hostile default; bounded output; signed origin receipts without truth overclaim; late/fenced facts cannot become current; OS support separated from native build support.

## Dependencies and gates

ADRs 005, 006, 007, 015. P0-02, P0-05, P0-07. Implementation status: NOT_AUTHORIZED.

## Review handoff

Provide escape/exfiltration/reset/process/resource tests for every OS/backend, capability digests, process-tree cancel behavior, enrollment/generation lifecycle, and reproducible environment receipts.

