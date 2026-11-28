# PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
"""Distinct monitored pilot; no verified rate/cancel bound and no strict gate relabeling."""
from resource_ledger import LedgerError, validate_snapshot, _validated_paths

# Adopted W03-T000-MONITORED-INSTALL-PILOT-POLICY-PROPOSAL.md §§4–5.
GIB = 2**30
POLICY = "monitored-pilot-r7"
# Explicit opt-in version (owner five-second decision): ONLY the full-observation age/response budget differs.
FULL5_POLICY = "monitored-pilot-full5-v1"
ARCHIVE5_POLICY = "monitored-pilot-archive5-v1"
FULL5_SECONDS = 5
NOMINAL = {"disk_cap_bytes": 64*GIB, "scratch_cap_bytes": 40*GIB,
           "aggregate_cap_bytes": 136*GIB, "preinstall_free_bytes": 125*GIB,
           "reserve_bytes": 20*GIB}
EARLY_FREE = 28*GIB
EARLY_AGGREGATE = 132*GIB
EARLY_SCRATCH = 36*GIB


def full5_budget(*values):
    # Exactly five seconds each; bool, NaN, infinity and any other value never qualify.
    return bool(values) and all(type(v) in (int, float) and v == FULL5_SECONDS for v in values)


def evaluate_pilot(snapshot, limits, config, *, now, preflight):
    return _decide(_measure(snapshot, limits, config, now=now, preflight=preflight), preflight)


def evaluate_pilot_pair(snapshot, limits, config, *, now):
    """(live, preflight) decisions from ONE complete validation at one `now`.

    Each equals evaluate_pilot(..., now=now, preflight=False/True); only the repeated
    validation/accounting of the same immutable snapshot is shared.
    """
    measured = _measure(snapshot, limits, config, now=now, preflight=False)
    return _decide(measured, False), _decide(measured, True)


def _measure(snapshot, limits, config, *, now, preflight):
    measured_paths = _validated_paths(snapshot, limits, now)  # = validate_snapshot(); keeps its path set
    policy = config.get("policy")
    if policy != POLICY and policy != FULL5_POLICY:  # GUARD_POLICY_SELECTOR
        if policy != ARCHIVE5_POLICY:raise LedgerError("explicit_pilot_policy_required")
    if type(preflight) is not bool:
        raise LedgerError("malformed_preflight")
    if limits.assumptions_verified or config.get("assumptionsVerified") is not False:
        raise LedgerError("pilot_cannot_claim_verified_bounds")
    if any(getattr(limits, key) != value for key, value in NOMINAL.items()):
        raise LedgerError("pilot_nominal_limits_changed")
    if policy == POLICY:
        if limits.max_sample_age_seconds > 2 or limits.max_response_seconds > 2:
            raise LedgerError("pilot_observation_budget_exceeds_policy")
    # The supplied limits and the selected config must agree on exactly five seconds; no mixed 2/5 tuple.
    elif not full5_budget(limits.max_sample_age_seconds, limits.max_response_seconds, config.get("fullDeadline"),
                          config.get("maxSampleAgeSeconds"), config.get("maxResponseSeconds")):  # GUARD_FULL5_BUDGET
        raise LedgerError("pilot_full5_budget_mismatch")
    if not hasattr(snapshot, "discovered_related_roots"):
        raise LedgerError("pilot_requires_complete_live_snapshot")
    for key, actual in (("relatedRootParent", snapshot.related_root_parent),
                        ("relatedRootPrefix", snapshot.related_root_prefix),
                        ("relatedRootAdditionalNames", list(snapshot.related_root_additional_names)),
                        ("knownWriterPaths", list(snapshot.known_writer_paths))):
        expected = config.get(key)
        if isinstance(expected, tuple):
            expected = list(expected)
        if key == "knownWriterPaths" and isinstance(expected, list):
            expected = sorted(expected)
        if expected != actual:
            raise LedgerError("pilot_snapshot_config_mismatch:" + key)
    if not set(snapshot.discovered_related_roots).issubset(snapshot.measured_roots):
        raise LedgerError("incomplete_related_roots")
    owned = config.get("ownedRoots")
    if not isinstance(owned, dict) or not {"lab", "r4", "r5", "r6", "r7"}.issubset(owned):
        raise LedgerError("missing_or_ambiguous_owned_roots")
    expected_roots = set(owned.values()) | set(snapshot.discovered_related_roots)
    if policy == ARCHIVE5_POLICY:
        from collection_control import validate_coverage
        validate_coverage(snapshot, config)
        expected_roots = set(snapshot.required_roots)
    if expected_roots != set(snapshot.required_roots):
        raise LedgerError("pilot_root_coverage_mismatch")
    # measured_paths is exactly {p for e in snapshot.entries for p in e.paths}, built once above.
    known = set(snapshot.known_writer_paths)
    if known and not known.issubset(measured_paths):
        raise LedgerError("unmeasured_known_writer")
    disk, scratch, total = snapshot.disk_allocated, snapshot.scratch_allocated, snapshot.allocated_bytes
    disk_size = next(e.size for e in snapshot.entries if e.category == "disk")
    remaining = max(0, limits.disk_cap_bytes - disk) + max(0, limits.scratch_cap_bytes - scratch)
    projected_free, projected_total = snapshot.free_bytes - remaining, total + remaining
    reasons = []
    if disk > limits.disk_cap_bytes or disk_size > limits.disk_cap_bytes:
        reasons.append("disk_cap_breach")
    if scratch > limits.scratch_cap_bytes:
        reasons.append("scratch_cap_breach")
    if total > limits.aggregate_cap_bytes or projected_total > limits.aggregate_cap_bytes:
        reasons.append("aggregate_plan_breach")
    if snapshot.free_bytes < limits.reserve_bytes:
        reasons.append("reserve_floor_breach")
    return snapshot, limits, policy, tuple(reasons), remaining, projected_free, projected_total, total, scratch


def _decide(measured, preflight):
    snapshot, limits, policy, reasons, remaining, projected_free, projected_total, total, scratch = measured
    reasons = list(reasons)
    if preflight and snapshot.free_bytes < limits.preinstall_free_bytes:
        reasons.append("preinstall_free_shortfall")
    # Equality permits admission; a live equality is the first early cancel trigger.
    def lower(value, threshold):
        return value < threshold if preflight else value <= threshold

    def upper(value, threshold):
        return value > threshold if preflight else value >= threshold

    if lower(projected_free, EARLY_FREE):
        reasons.append("projected_free_early_trigger")
    if lower(snapshot.free_bytes, EARLY_FREE):
        reasons.append("actual_free_early_trigger")
    if upper(total, EARLY_AGGREGATE) or upper(projected_total, EARLY_AGGREGATE):
        reasons.append("aggregate_early_trigger")
    if upper(scratch, EARLY_SCRATCH):
        reasons.append("scratch_early_trigger")
    def policy_result():
        return {"policy": policy, "admitted": not reasons, "reasons": reasons,
            "remaining_bytes": remaining, "projected_allocation_bytes": projected_total,
            "projected_free_bytes": projected_free, "assumptionsVerified": False,
            "hardQuotaOrConfinementClaim": False, "verifiedMaximumBoundClaim": False,
            "free_gap_bytes": max(0, EARLY_FREE - projected_free),
            "aggregate_gap_bytes": max(0, projected_total - EARLY_AGGREGATE),
            "runway_bytes": None}
    result=policy_result()

    if policy == ARCHIVE5_POLICY:
        result.update(historicalEvidenceMode='verified_archive_capsules',
                      catalogSHA256=snapshot.archive_catalog_digest,
                      liveRootCount=len(snapshot.required_roots)-1,
                      capsuleCount=len(snapshot.archive_capsule_paths),
                      migratedAbsentCount=len(snapshot.archive_absent_roots))
    return result
