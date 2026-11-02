# PROTOTYPE — DISPOSABLE — NOT PRODUCTION (Mastercoder freeze-closure)
"""Run-bound event validation and sticky ambiguity; helper tests are PLUMBING_ONLY."""
import math
class ProtocolError(ValueError): pass
class Outcome:
    def __init__(self, nonce, pid, owner_start=None, *, operation="reinstall-existing"):
        self.nonce,self.pid,self.instance,self.seq=nonce,pid,None,0
        self.operation=operation;self.begin_command=None
        self.owner_start=owner_start; self.finish_command=None; self.finish_ack=None
        self.started=False; self.callback=None; self.stopped=False; self.exact_stop=False; self.cancel=False
        self.unknown=[]; self.breaches=[]; self.settled_since=None; self.fingerprint=None
    def fault(self, reason):
        if reason not in self.unknown: self.unknown.append(reason)
        self.settled_since=None
    def event(self,e):
        if not isinstance(e,dict) or e.get('nonce')!=self.nonce or type(e.get('pid')) is not int or e['pid']!=self.pid:
            raise ProtocolError('event_identity')
        owner_start=e.get('ownerStart')
        if not isinstance(owner_start,str) or not owner_start or (self.owner_start is not None and owner_start!=self.owner_start):
            raise ProtocolError('event_owner_start_identity')
        if type(e.get('seq')) is not int or e['seq']!=self.seq+1: raise ProtocolError('event_sequence')
        name=e.get('event')
        if type(name) is not str: raise ProtocolError('event_type')
        instance=e.get('instance')
        if self.instance is not None and instance!=self.instance: raise ProtocolError('event_object_identity')
        if name=='ready':
            if self.instance is not None or not isinstance(instance,str) or not instance or e.get('stateName')!='stopped': raise ProtocolError('ready_not_exact_stopped')
            self.instance=instance
        if name in ('install_begin','boot_begin'):
            if name!=('boot_begin' if self.operation=='baseline' else 'install_begin'):raise ProtocolError('wrong_operation_begin')
            if self.begin_command is None and self.operation=='baseline':raise ProtocolError('unsolicited_begin')
            if self.instance is None or self.started or self.finish_command is not None: raise ProtocolError('duplicate_or_unbound_begin')
            self.started=True
        if name in ('install_callback_success','install_callback_error','boot_callback_success','boot_callback_error'):
            if name.startswith('boot_')!=(self.operation=='baseline'):raise ProtocolError('wrong_operation_callback')
            if not self.started or self.callback is not None: raise ProtocolError('callback_without_start_or_duplicate')
            self.callback=name
        if name=='cancel_requested':
            if self.cancel: raise ProtocolError('duplicate_cancel')
            self.cancel=True
        if name=='exact_owner_stopped_after_callback':
            if self.callback is None or e.get('stateName')!='stopped':raise ProtocolError('exact_stop_without_callback_or_state')
            self.exact_stop=True
        if name=='finish_ack':
            if (self.finish_command is None or self.finish_ack is not None or e.get('command')!='finish'
                    or type(e.get('commandSeq')) is not int or e['commandSeq']!=self.finish_command
                    or self.instance is None or e.get('stateName')!='stopped'
                    or (self.started and (self.callback is None or not self.exact_stop))):
                raise ProtocolError('unbound_or_ineligible_finish_ack')
            self.finish_ack=e['commandSeq']
        if name=='transport_fault': self.fault('owner_transport_fault')
        if 'stateName' in e:
            if e['stateName'] not in ('running','stopped','starting','stopping','paused','pausing','resuming','saving','restoring','error','unknown'): raise ProtocolError('native_state_enum')
            self.stopped=e['stateName']=='stopped'
            if e['stateName'] in ('error','unknown'): self.fault('unknown_native_state')
        self.owner_start=owner_start; self.seq=e['seq']
    def expect_begin(self,command_seq):
        if self.started or self.begin_command is not None:raise ProtocolError('duplicate_begin_command')
        self.begin_command=command_seq

    def expect_finish(self, command_seq):
        if type(command_seq) is not int or command_seq<=0 or self.finish_command is not None:
            raise ProtocolError('invalid_or_duplicate_finish_command')
        self.finish_command=command_seq
    def settle(self, fingerprint, now, seconds, resource_breaches):
        self.breaches=list(resource_breaches)
        if self.unknown or self.breaches or self.callback is None or not self.stopped or not self.exact_stop:
            self.settled_since=None; return None
        if self.fingerprint!=fingerprint or self.settled_since is None:
            self.fingerprint=fingerprint;self.settled_since=now;return None
        if now-self.settled_since < seconds: return None
        return 'SUCCESS' if self.callback in ('install_callback_success','boot_callback_success') and not self.cancel else 'CANCELLED' if self.cancel else 'FAIL'
    def record(self):
        return dict(nonce=self.nonce,pid=self.pid,ownerStart=self.owner_start,instance=self.instance,seq=self.seq,started=self.started,callback=self.callback,
                    finishCommandSeq=self.finish_command,finishAckCommandSeq=self.finish_ack,
                    exactStopped=self.stopped,exactStopEvent=self.exact_stop,cancelAcknowledged=self.cancel,unknown=self.unknown,breaches=self.breaches,
                    assumptionsVerified=False,hardGuarantee=False)
