"""Delivers the operator's interrupt to the driver, on a platform that cannot send one directly.

Windows disables CTRL_C_EVENT for a process group created with CREATE_NEW_PROCESS_GROUP, and sending
it to the shared console would interrupt the measuring process too. So the parent sends
CTRL_BREAK_EVENT to the driver's own group, and this launcher translates it into the KeyboardInterrupt
that an operator pressing Ctrl+C in a console raises. Nothing else about the run changes: the driver
is executed under its own __main__ with its own argument vector.
"""
import runpy
import signal
import sys


def raise_keyboard_interrupt(signum, frame):
    raise KeyboardInterrupt


signal.signal(signal.SIGBREAK, raise_keyboard_interrupt)
driver = sys.argv[1]
sys.argv = [driver] + sys.argv[2:]
runpy.run_path(driver, run_name="__main__")
