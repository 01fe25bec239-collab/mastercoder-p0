# PROTOTYPE — DISPOSABLE — NOT PRODUCTION.
import sys, unittest
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import patch
sys.path.insert(0, str(Path(__file__).resolve().parents[1] / 'source'))
import native_control as nc

class Heartbeat(unittest.TestCase):
    def setUp(self):
        self.t = 10.0
        self.sent = []
        self.faults = []
        self.h = nc.NativeSupervisor.__new__(nc.NativeSupervisor)
        self.h.archive_mode = lambda: False
        self.h.monitor_done = False
        self.h.children = {'owner': SimpleNamespace(stdout=object(), poll=lambda: None), 'monitor': SimpleNamespace(stdout=object())}
        self.h.owner_state = SimpleNamespace(instance='owner', owner_start=1)
        self.h.prebegin = True
        self.h.owner_handoff_sent = False
        self.h.owner_handoff_requested = False
        self.h.finish_attempted = False
        self.h.next_owner_heartbeat = 0
        self.h.n = {'controlInterval': .25}
        self.h.c = {'run_bytes': 65536, 'output_bytes': 65536}
        self.h.owner_command_send = self.sent.append
        self.h.gate = SimpleNamespace(fault=self.faults.append)
        self.clock = patch.object(nc.time, 'monotonic', side_effect=lambda: self.t)
        self.select = patch.object(nc.select, 'select', return_value=([], [], []))
        self.clock.start(); self.select.start()
        self.addCleanup(self.clock.stop); self.addCleanup(self.select.stop)

    def test_cadence_handoff_and_stops(self):
        for _ in range(100): self.h.pump(0)
        self.assertEqual(self.sent, ['heartbeat'])
        self.assertEqual(self.h.next_owner_heartbeat, 10.25)
        self.t = 10.25; self.h.pump(0)
        self.assertEqual(self.sent, ['heartbeat', 'heartbeat'])
        self.h.owner_state.instance = None
        self.h.owner_handoff_requested = True
        self.h.pump(0)
        self.assertEqual(self.sent[-1], 'handoff')
        self.h.owner_handoff_sent = True
        self.h.pump(0)
        self.assertEqual(len(self.sent), 3)
        self.h.owner_state.instance = 'owner'; self.h.finish_attempted = True
        self.t = 11; self.h.pump(0)
        self.h.finish_attempted = False; self.h.children['owner'].poll = lambda: 0
        self.h.pump(0)
        self.assertEqual(len(self.sent), 3)

    def test_write_error_fault_and_timer_not_advanced(self):
        def fail(_): raise OSError('failed')
        self.h.owner_command_send = fail
        self.h.pump(0)
        self.assertEqual(self.faults, ['owner_control_lost:failed'])
        self.assertEqual(self.h.next_owner_heartbeat, 0)

    def test_archive_delegates_unchanged(self):
        self.h.archive_mode = lambda: True
        self.h.pump_archive = lambda duration: ('archive', duration)
        self.assertEqual(self.h.pump(.2), ('archive', .2))
        self.assertEqual(self.sent, [])

if __name__ == '__main__': unittest.main()
