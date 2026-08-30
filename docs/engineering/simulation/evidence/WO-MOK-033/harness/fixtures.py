#!/usr/bin/env python3
"""Fixture record streams, built independently of the instruments' own test fixtures.

Every figure a cross-check reads is derived here from the event list rather than stated beside it, so
a fixture is consistent by construction and a disagreement has to be introduced on purpose. That is
what makes `VER-MOK-019` B17's seven cases each break exactly one equality.
"""

from __future__ import annotations

import json
import os

ROSTER = 12


def header(engine="0.1.0", ticks=50, policy="social", density="0.75", seed=0):
    return {
        "record": "header",
        "schema": 3,
        "engine": engine,
        "config": {
            "seed": seed,
            "ticks": ticks,
            "policy": policy,
            "density": density,
            "trace_actions": False,
        },
    }


def event(name, tick=1, subject="M01", result=None):
    record = {"record": "event", "tick": tick, "event": name, "subject": subject}
    if result is not None:
        record["result"] = result
    return record


def stream(events=(), engine="0.1.0", ticks=50, policy="social", density="0.75", seed=0,
           roster=ROSTER, reason="tick_limit", run_ticks=None, deaths=None,
           run_overrides=None, drop_run_keys=(), drop_records=()):
    """Build a stream whose run record agrees with its events on all seven of rule 10.1's equalities."""
    body = [event("agent_initialized", subject=f"M{index:02d}") for index in range(1, roster + 1)]
    body.extend(events)

    tally = {}
    for record in body:
        if record.get("record") == "event":
            tally[record["event"]] = tally.get(record["event"], 0) + 1

    died = tally.get("agent_died", 0) if deaths is None else deaths
    consumed_total = tally.get("food_consumed", 0)
    skipped_total = tally.get("food_regeneration_skipped", 0)

    run = {
        "record": "run",
        "reason": reason,
        "ticks": ticks if run_ticks is None else run_ticks,
        "survivors": roster - died,
        "deaths": died,
        "crossings": tally.get("territory_crossed", 0),
        "consumed": {"low": consumed_total, "medium": 0, "high": 0},
        "regenerated": tally.get("food_regenerated", 0),
        "regeneration_skipped": {"depleted": 0, "capacity": skipped_total},
        "final": {
            "territories": {
                "A": {"population": (roster - died + 1) // 2, "low": 8, "medium": 8, "high": 8},
                "B": {"population": (roster - died) // 2, "low": 8, "medium": 8, "high": 8},
            }
        },
        "agents": [
            {"id": f"M{index:02d}", "name": f"N{index:02d}", "territory": "A",
             "died_at": 10 if index <= died else None}
            for index in range(1, roster + 1)
        ],
    }
    if run_overrides:
        run.update(run_overrides)
    for key in drop_run_keys:
        run.pop(key, None)

    records = [header(engine=engine, ticks=ticks, policy=policy, density=density, seed=seed)]
    records.extend(body)
    records.append(run)
    records = [record for record in records if record.get("record") not in drop_records]

    return b"".join(
        json.dumps(record, ensure_ascii=False).encode("utf-8") + b"\n" for record in records
    )


CONFLICTS = [
    event("attack_resolved", result={"damage": 5, "target_died": "no"}),
    event("attack_resolved", result={"damage": 0, "target_died": "no"}),
    event("attack_resolved", result={"damage": 9, "target_died": "yes"}),
    event("threat_resolved", result={"increase": 4}),
    event("threat_resolved", result={"increase": 0}),
    event("surrender_resolved", result={"transferred": 2}),
    event("surrender_resolved", result={"transferred": 0}),
]

BODY = [
    event("territory_crossed"),
    event("territory_crossed"),
    event("food_consumed"),
    event("food_consumed"),
    event("food_consumed"),
    event("food_regenerated"),
    event("food_regeneration_skipped"),
    event("agent_died"),
] + CONFLICTS


def write(directory, name, raw):
    path = os.path.join(directory, name)
    with open(path, "wb") as handle:
        handle.write(raw)
    return path
