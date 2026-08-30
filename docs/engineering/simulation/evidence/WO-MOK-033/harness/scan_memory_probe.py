#!/usr/bin/env python3
"""Measure what one call to the driver's `scan_file` adds to a process's memory high-water.

`VER-MOK-019`'s performance bullet asks for a stream materially larger than `ADR-MOK-008`'s measured
2,723,014 bytes, "to establish that the driver streams rather than reading a whole file into memory".
An end-to-end batch answers that only indirectly: its process also starts an interpreter, spawns an
engine and captures its output, and a CPython interpreter's own working set is larger than the stream
being read. So this probe isolates the one function whose memory behaviour is the claim.

It runs in a fresh process per stream, records the working set once the driver is imported and before
the scan, calls `scan_file` on the stream, and records the process's lifetime peak afterwards. A
reader that held the file would show a peak at least the stream's size above that baseline, and a
reader that parsed it whole would show several times the stream's size, because a parsed JSON record
is larger than its bytes. The probe imports the repository's own file and does not copy or alter it.
"""

from __future__ import annotations

import ctypes
import ctypes.wintypes as wintypes
import json
import os
import sys

REPO = "C:/Users/mathi/Mokiterions-understand-20260829-1340"
sys.path.insert(0, f"{REPO}/scripts")


class PROCESS_MEMORY_COUNTERS(ctypes.Structure):
    _fields_ = [
        ("cb", wintypes.DWORD),
        ("PageFaultCount", wintypes.DWORD),
        ("PeakWorkingSetSize", ctypes.c_size_t),
        ("WorkingSetSize", ctypes.c_size_t),
        ("QuotaPeakPagedPoolUsage", ctypes.c_size_t),
        ("QuotaPagedPoolUsage", ctypes.c_size_t),
        ("QuotaPeakNonPagedPoolUsage", ctypes.c_size_t),
        ("QuotaNonPagedPoolUsage", ctypes.c_size_t),
        ("PagefileUsage", ctypes.c_size_t),
        ("PeakPagefileUsage", ctypes.c_size_t),
    ]


_PSAPI = ctypes.WinDLL("psapi", use_last_error=True)
_KERNEL = ctypes.WinDLL("kernel32", use_last_error=True)


def own_memory():
    counters = PROCESS_MEMORY_COUNTERS()
    counters.cb = ctypes.sizeof(counters)
    handle = _KERNEL.GetCurrentProcess()
    if not _PSAPI.GetProcessMemoryInfo(wintypes.HANDLE(handle), ctypes.byref(counters),
                                       counters.cb):
        raise ctypes.WinError(ctypes.get_last_error())
    return counters.WorkingSetSize, counters.PeakWorkingSetSize


def main():
    path = sys.argv[1]
    import run_simulation_batch

    before_current, before_peak = own_memory()
    scan = run_simulation_batch.scan_file(path)
    after_current, after_peak = own_memory()

    # The baseline is the larger of the two readings taken before the scan: the peak may already be
    # above the current working set from the interpreter's own start-up and the import.
    baseline = max(before_current, before_peak)
    size = os.path.getsize(path)
    print(json.dumps({
        "stream_bytes": size,
        "records": sum(scan["counted"].values()),
        "digest": scan["digest"][:16] + "...",
        "baseline_current": before_current,
        "baseline_peak": before_peak,
        "after_current": after_current,
        "after_peak": after_peak,
        "added_peak": after_peak - baseline,
        "added_per_stream_byte": round((after_peak - baseline) / size, 4),
    }))
    return 0


if __name__ == "__main__":
    sys.exit(main())
