# Security policy (internal, private prototype repository)

- Report any suspected secret exposure, sandbox escape, unexpected egress, cross-tenant access or lost device **immediately as an incident (master)**, then follow `TROUBLESHOOTING.md → SECURITY INCIDENT` in your program package: **rotate first**, notify within 1 hour, never rewrite history alone, invalidate affected evidence.
- Never paste real secrets into issues, PRs, commits, agent chats or evidence. Use canaries and the documented fake example values only.
- The public open-source repository (future) will have its own SECURITY.md with private vulnerability reporting (see COMMON/OPEN_SOURCE_STRATEGY.md).
