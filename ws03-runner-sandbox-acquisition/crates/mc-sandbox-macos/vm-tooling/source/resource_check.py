# PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
"""Complete live admission; rejected plans never execute any native command."""
from dataclasses import asdict, replace
import json
import math
from pathlib import Path
import sys
import time
import native_config
from types import SimpleNamespace
a=SimpleNamespace(C=native_config.load_native(Path(__file__).parent / "native-config.json"), E=Path(__file__).parent)
from resource_ledger import LedgerPaths, ResourceLimits, collect_snapshot, evaluate

def paths():
    c = a.C
    roots = {key: Path(value) for key, value in c['ownedRoots'].items()}
    retained = json.loads((a.E / 'ledger-roots.json').read_text())
    parent = Path(retained['discoveryParent'])
    found = {str(p) for p in parent.iterdir() if p.name.startswith(retained['discoveryPrefix']) or p.name in retained['additionalNames']}
    if found != set(retained['discoveredRoots']):
        raise ValueError('stale_or_incomplete_retained_root_inventory')
    for number, root in enumerate(retained['discoveredRoots']):
        roots['retained-' + str(number)] = Path(root)
    temp = Path(c['frameworkTempRoot'])
    return LedgerPaths(roots, Path(c['imagePath']), Path(c['bundlePath']) / 'Disk.img', temp,
        tuple(temp / name for name in c['preservedFrameworkNames']), c['expectedUID'], c['frameworkPrefix'])

def limits():
    c = a.C
    return ResourceLimits(c['disk'], c['scratch'], c['aggregate'], c['preinstallFree'], c['freeFloor'],
        c['lossRateBytesPerSecond'], c['maxResponseSeconds'], c['stopDeadline'], c['maxSampleAgeSeconds'],
        c['settlementSeconds'], c['assumptionsVerified'], c['assumptionBasis'])

def observation():
    snapshot = collect_snapshot(paths(), clock=time.monotonic)
    decision = evaluate(snapshot, limits(), now=time.monotonic(), preflight=True)
    if not response_budget_fits(snapshot.finished - snapshot.started, a.C['monitorInterval'], a.C['maxResponseSeconds']):
        decision = replace(decision, admitted=False,
            reasons=decision.reasons + ('measured_collection_plus_poll_exceeds_response_budget',))
    return snapshot, decision

def response_budget_fits(collection_seconds, poll_seconds, response_seconds):
    for value in (collection_seconds, poll_seconds, response_seconds):
        if type(value) not in (int, float) or not math.isfinite(value) or value < 0:
            raise ValueError('malformed_response_budget')
    if response_seconds == 0:
        raise ValueError('malformed_response_budget')
    return collection_seconds + poll_seconds <= response_seconds

def admission(name):
    snapshot, decision = observation()
    prefix = a.C['frameworkPrefix']
    framework = [e for e in snapshot.entries if any(prefix in p for p in e.paths)]
    image = [e for e in snapshot.entries if e.category == 'image']
    value = {'snapshot': snapshot.to_dict(), 'decision': decision.to_dict(), 'limits': asdict(limits()),
        'collectionSeconds': snapshot.finished - snapshot.started,
        'observedCollectionPlusPollSeconds': snapshot.finished - snapshot.started + a.C['monitorInterval'],
        'configuredResponseBudgetSeconds': a.C['maxResponseSeconds'],
        'frameworkAllocatedBytes': sum(e.allocated for e in framework),
        'imageAllocatedBytes': sum(e.allocated for e in image),
        'requiredActualFreeBytesWithRunway': decision.remaining_bytes + limits().reserve_bytes + decision.runway_bytes,
        'preinstall125GiBMet': snapshot.free_bytes >= limits().preinstall_free_bytes,
        'nativeInstallation': 'NOT_RUN', 'nativeAttempts': 0,
        'scopedAccountingOnly': True, 'hardQuotaOrConfinementClaim': False,
        'reason': 'Every preflight gate is mandatory. Failed capacity or unsupported loss/latency assumptions denies native execution.'}
    a.save(name, value)
    print(json.dumps({k: v for k, v in value.items() if k not in ('snapshot', 'limits')}, indent=2), flush=True)
    return value

if __name__ == '__main__':
    admission(sys.argv[1])
