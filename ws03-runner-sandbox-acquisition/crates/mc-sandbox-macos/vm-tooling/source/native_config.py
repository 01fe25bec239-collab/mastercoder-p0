# PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
"""Explicit integration policy. No environment, assets or native APIs are consulted."""
import json, math
from pathlib import Path
from config import plain_path
from pilot_policy import FULL5_POLICY, full5_budget, ARCHIVE5_POLICY
from resource_ledger import ResourceLimits

def load_native(path, *, prebegin=False, operational=False):
    c=json.loads(plain_path(path, private=True).read_text())
    if not prebegin and not operational and (c.get('nativeMode') is not None or c.get('nativePrebeginAllowed',False) is not False):
        raise ValueError('explicit_prebegin_profile_required')
    if not operational and c['nativeInstallationAllowed'] is not False:  # GUARD_NATIVE_GATE
        raise ValueError('native_dispatch_disabled')
    if c['assumptionsVerified'] is not False:
        raise ValueError('unverified_assumptions_required')
    for key in ('evidence','lab','imagePath','bundlePath','frameworkTempRoot','relatedRootParent'):
        p=Path(c[key])
        if not p.is_absolute() or str(p)!=str(p.absolute()): raise ValueError('config_path:'+key)
    if not {'lab','r4','r5','r6','r7','r11','r12'}.issubset(c['ownedRoots']):
        raise ValueError('incomplete_owned_roots')
    if c['ownedRoots']['r13' if prebegin else 'r12']!=c['evidence'] or c['knownWriterPaths'] or c['fixtureSymlinkRecords']:
        raise ValueError('fresh_authority_required')
    for key in ('fastDeadline','fullDeadline','settlementSeconds','stopDeadline','installDeadline'):
        if type(c[key]) not in (int,float) or not math.isfinite(c[key]) or c[key]<=0:
            raise ValueError('duration:'+key)
    full5=c.get('policy')==FULL5_POLICY  # inherited selector/mutation seam stays exact
    if c.get('policy')==ARCHIVE5_POLICY:full5=True
    if c['fastDeadline']>.5 or (not full5 and c['fullDeadline']>2) or c['stopDeadline']!=30 or c['installDeadline']!=1800:  # GUARD_LEGACY_FULL_DEADLINE
        raise ValueError('policy_deadline')
    if full5 and not full5_budget(c['fullDeadline'],c.get('maxSampleAgeSeconds'),c.get('maxResponseSeconds')):  # GUARD_FULL5_CONFIG
        raise ValueError('full5_policy_deadline')
    expected={'disk':64,'scratch':40,'aggregate':136,'preinstallFree':125,'freeFloor':20}
    if any(type(c[k]) is not int or c[k]!=v*2**30 for k,v in expected.items()):
        raise ValueError('policy_limit')
    interval,age=c['controlInterval'],c['controlHeartbeatMaxAge']
    if (type(interval) not in (int,float) or not math.isfinite(interval) or not 0<interval<=.5
            or type(age) not in (int,float) or not math.isfinite(age) or age<=interval):
        raise ValueError('native_control_cadence')
    special_keys = ('retainedMetadataSnapshotPath', 'retainedMetadataExpectedSHA256')
    if any(k in c for k in special_keys):
        pin = c.get(special_keys[1])
        if (not all(k in c for k in special_keys) or not Path(c[special_keys[0]]).is_absolute()
                or type(pin) is not str or len(pin) != 64 or any(ch not in '0123456789abcdef' for ch in pin)
                or 'currentControlSocket' in c):
            raise ValueError('invalid_special_metadata_config')
    if c.get('policy')==ARCHIVE5_POLICY:
        validate_archive_config(c)
    if prebegin:
        validate_prebegin(c)
    if operational:
        validate_operational(c)
    return c

def validate_prebegin(c):
    if (c.get('nativeMode')!='prebegin-existing' or c.get('nativePrebeginAllowed') is not True
            or c.get('nativeInstallationAllowed') is not False):
        raise ValueError('prebegin_permission')
    duration=c.get('preparationDeadline')
    if type(duration) not in (int,float) or not math.isfinite(duration) or not 0<duration<=300:
        raise ValueError('preparation_deadline')
    if c.get('prebeginAuxiliaryMaxBytes')!=64*2**20:
        raise ValueError('prebegin_auxiliary_limit')
    if not isinstance(c.get('ownedRoots'),dict) or c['ownedRoots'].get('r13')!=c.get('evidence'):
        raise ValueError('prebegin_owned_root')
    return c

def validate_operational(c):
    if (c.get('nativeMode') not in ('reinstall-existing','baseline')
            or c.get('nativePrebeginAllowed',False) is not False
            or c.get('nativeInstallationAllowed') is not True
            or c.get('assumptionsVerified') is not False or c.get('policy')!=FULL5_POLICY):
        raise ValueError('operational_permission')
    duration=c.get('preparationDeadline')
    if type(duration) not in (int,float) or not math.isfinite(duration) or not 0<duration<=300:
        raise ValueError('preparation_deadline')
    if c['controlHeartbeatMaxAge']>3:raise ValueError('operational_liveness')
    if c['nativeMode']=='baseline':validate_baseline(c)
    elif c.get('installedBaseline') is not False:raise ValueError('uninstalled_baseline_required')
    return c


def validate_baseline(c):
    from config import digest
    if c.get('installedBaseline') is not True:raise ValueError('baseline_not_verified')
    duration=c.get('runDeadline')
    if type(duration) not in (int,float) or not math.isfinite(duration) or duration<=0:
        raise ValueError('baseline_deadline')
    # A separately verified current manifest binds the installation receipt to the post-install assets.
    path=plain_path(c['installedBaselineEvidencePath'],private=True)
    if digest(path)!=c['installedBaselineEvidenceSHA256']:raise ValueError('baseline_evidence_pin')
    evidence=json.loads(path.read_text())
    if (evidence.get('expectedManifestSHA256')!=c['expectedManifestSHA256']
            or evidence.get('installedBaseline') is not True):raise ValueError('baseline_manifest_binding')
    receipt_path=plain_path(evidence['installationReceiptPath'],private=True)
    if digest(receipt_path)!=evidence['installationReceiptSHA256']:raise ValueError('installation_receipt_pin')
    receipt=json.loads(receipt_path.read_text());owner=receipt.get('owner',{})
    if (receipt.get('outcome')!='INSTALLED' or receipt.get('control_clean') is not True
            or receipt.get('unknown')!=[] or receipt.get('native_state')!='EXACT_STOPPED'
            or receipt.get('operation')!='reinstall-existing' or owner.get('callback')!='install_callback_success'
            or owner.get('exactStopEvent') is not True or owner.get('finishAckCommandSeq') is None
            or owner.get('cancelAcknowledged') is not False or owner.get('unknown')!=[]
            or owner.get('breaches')!=[] or set(receipt.get('child_waits',{}))!={'owner','monitor','recorder'}
            or any(v!=0 for v in receipt['child_waits'].values())):
        raise ValueError('installation_not_proved')
    return evidence


def validate_monitor_access(c):
    # Explicit pre-begin admits observation only; operational observation grants no begin.
    if c.get('nativeMode') in ('reinstall-existing','baseline'):return validate_operational(c)
    if c.get('nativeMode') is not None or c.get('nativePrebeginAllowed',False) is not False:
        return validate_prebegin(c)
    if c.get('nativeInstallationAllowed') is not True:
        raise ValueError('native_installation_disabled')
    return c

def limits(c):
    return ResourceLimits(c['disk'],c['scratch'],c['aggregate'],c['preinstallFree'],c['freeFloor'],c['lossRateBytesPerSecond'],c['maxResponseSeconds'],c['stopDeadline'],c['maxSampleAgeSeconds'],c['settlementSeconds'],False,c['assumptionBasis'])

def native_argv(c,config):
    if c.get('nativeMode') in ('reinstall-existing','baseline'):
        validate_operational(c)
        return [str(Path(c['evidence'])/'VMOwner'),c['nativeMode'],str(config)]
    if c.get('nativeMode') is not None or c.get('nativePrebeginAllowed') is True:
        validate_prebegin(c)
        return [str(Path(c['evidence'])/'VMOwner'),'prebegin-existing',str(config)]
    if c['nativeInstallationAllowed'] is not True:  # GUARD_NATIVE_DISPATCH
        raise ValueError('native_dispatch_disabled')
    raise ValueError('explicit_operational_profile_required')

# R37 accounting contract: finite refusal limits, no operational allocation grants.
ARCHIVE_CAPS=dict(catalogBytes=4*2**20,capsules=128,capsuleBytes=128*2**20,
                  archiveAllocatedBytes=8*2**30,livePaths=65536,liveInodes=65536,
                  depth=256,pathBytes=16*2**20)


def validate_archive_config(c):
    from archive_catalog import canonical,pin,member_name
    if c.get('policy')!=ARCHIVE5_POLICY or not full5_budget(c.get('fullDeadline'),c.get('maxSampleAgeSeconds'),c.get('maxResponseSeconds')):
        raise ValueError('archive_policy_budget')
    if c.get('assumptionsVerified') is not False or c.get('nativeInstallationAllowed') is not False:
        raise ValueError('archive_gates')
    a=c.get('archiveAccounting')
    fields={'directory','catalogName','catalogSHA256','capsuleSHA256','migrationRows','readerDischarges','caps','currentSourceRoot'}
    if type(a) is not dict or set(a)!=fields:
        raise ValueError('archive_config_fields')
    if type(a['caps']) is not dict or set(a['caps'])!=set(ARCHIVE_CAPS) or any(type(a['caps'][k]) is not int or a['caps'][k]!=v for k,v in ARCHIVE_CAPS.items()):
        raise ValueError('archive_config_caps')
    canonical(a['directory']);canonical(a['currentSourceRoot'])
    if '/' in member_name(a['catalogName']):raise ValueError('archive_catalog_name')
    pin(a['catalogSHA256'])
    if type(a['capsuleSHA256']) is not dict or not 1<=len(a['capsuleSHA256'])<=a['caps']['capsules']:
        raise ValueError('archive_config_capsules')
    for name,digest in a['capsuleSHA256'].items():
        if '/' in member_name(name) or name==a['catalogName']:raise ValueError('archive_capsule_name')
        pin(digest)
    if type(a['migrationRows']) is not list or type(a['readerDischarges']) is not list:
        raise ValueError('archive_config_migration')
    return a
