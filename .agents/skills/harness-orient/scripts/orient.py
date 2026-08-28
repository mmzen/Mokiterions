#!/usr/bin/env python3
"""Run the portable, read-only harness-orient procedure."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import stat
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Iterable, Mapping, Sequence


ORIENTATION_SCHEMA = "se-harness-orientation-result-v1"
RECEIPT_SCHEMA = "se-harness-execution-receipt-v1"
MANIFEST_SCHEMA = "se-harness-skill-manifest-v1"
CONTRACT_SCHEMA = "se-harness-skill-contract-v1"
TEXT_MODE = "utf8-text-lf-v1"
MINIMUM_VERSION = (0, 5, 0)
MAX_OUTPUT_BYTES = 1 << 20
MAX_STATE_FILES = 100_000
MAX_LAUNCHER_ARGUMENTS = 32
MAX_ARGUMENT_BYTES = 4096
ARTIFACT_ID = re.compile(r"[A-Z][A-Z0-9]*-[A-Z0-9]+-[0-9]+")
VERSION = re.compile(r"(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)(?:[-+][0-9A-Za-z.-]+)?")
SECRET_ASSIGNMENT = re.compile(r"(?i)\b(token|password|secret|authorization|credential)\s*[:=]\s*\S+")


class OrientationError(RuntimeError):
    """A bounded orientation diagnostic."""

    def __init__(self, code: str, message: str, *, outcome: str = "failed") -> None:
        super().__init__(f"{code}: {message}")
        self.code = code
        self.message = message
        self.outcome = outcome


@dataclass(frozen=True)
class CommandResult:
    operation: str
    returncode: int
    stdout: bytes
    stderr: bytes
    status: str

    def receipt_entry(self) -> dict[str, Any]:
        return {
            "exit_code": self.returncode,
            "id": self.operation,
            "status": self.status,
        }


@dataclass(frozen=True)
class TreeState:
    digest: str
    files: Mapping[str, str]


def _reject_duplicate_keys(pairs: Iterable[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise OrientationError("AEXORI002", f"duplicate JSON key: {key}")
        result[key] = value
    return result


def _json_bytes(value: Any) -> bytes:
    def validate(item: Any) -> None:
        if item is None or isinstance(item, (str, bool)):
            return
        if isinstance(item, int) and not isinstance(item, bool):
            if not -(1 << 63) < item < (1 << 63):
                raise OrientationError("AEXORI003", "canonical integer is outside the supported range")
            return
        if isinstance(item, float):
            raise OrientationError("AEXORI003", "floating-point values are not canonical")
        if isinstance(item, list):
            for child in item:
                validate(child)
            return
        if isinstance(item, dict):
            for key, child in item.items():
                if not isinstance(key, str):
                    raise OrientationError("AEXORI003", "canonical object keys must be strings")
                validate(child)
            return
        raise OrientationError("AEXORI003", "unsupported canonical JSON value")

    validate(value)
    return (
        json.dumps(value, ensure_ascii=False, allow_nan=False, separators=(",", ":"), sort_keys=True)
        + "\n"
    ).encode("utf-8")


def _json_object(raw: bytes, operation: str) -> dict[str, Any]:
    if len(raw) > MAX_OUTPUT_BYTES:
        raise OrientationError("AEXORI020", f"{operation} JSON exceeded the bounded output size")
    try:
        value = json.loads(raw.decode("utf-8"), object_pairs_hook=_reject_duplicate_keys)
    except OrientationError:
        raise
    except (UnicodeDecodeError, json.JSONDecodeError, RecursionError) as exc:
        raise OrientationError("AEXORI021", f"{operation} did not return valid bounded UTF-8 JSON") from exc
    if not isinstance(value, dict):
        raise OrientationError("AEXORI021", f"{operation} JSON must be an object")
    return value


def _version_tuple(value: str) -> tuple[int, int, int]:
    matched = VERSION.fullmatch(value)
    if matched is None:
        raise OrientationError("AEXORI004", f"invalid evaluator version: {value!r}", outcome="blocked")
    return tuple(int(matched.group(index)) for index in range(1, 4))


def _parse_launcher(raw: str) -> tuple[str, ...]:
    try:
        value = json.loads(raw, object_pairs_hook=_reject_duplicate_keys)
    except OrientationError:
        raise
    except (json.JSONDecodeError, RecursionError) as exc:
        raise OrientationError("AEXORI005", "evaluator launcher must be a JSON array") from exc
    if not isinstance(value, list) or not 1 <= len(value) <= MAX_LAUNCHER_ARGUMENTS:
        raise OrientationError("AEXORI005", "evaluator launcher must be a bounded non-empty JSON array")
    result: list[str] = []
    for item in value:
        if (
            not isinstance(item, str)
            or not item
            or len(item.encode("utf-8", "surrogatepass")) > MAX_ARGUMENT_BYTES
            or "\x00" in item
            or "\r" in item
            or "\n" in item
        ):
            raise OrientationError("AEXORI005", "evaluator launcher contains an invalid argument")
        result.append(item)
    return tuple(result)


def _safe_target(raw: str) -> Path:
    supplied = Path(raw).expanduser()
    if supplied.is_symlink():
        raise OrientationError("AEXORI006", "repository target must not be a symlink", outcome="blocked")
    try:
        target = supplied.resolve(strict=True)
    except OSError as exc:
        raise OrientationError("AEXORI006", "repository target does not exist", outcome="blocked") from exc
    if not target.is_dir():
        raise OrientationError("AEXORI006", "repository target is not a directory", outcome="blocked")
    return target


def _entry_kind(path: Path) -> str:
    try:
        mode = path.lstat().st_mode
    except OSError as exc:
        raise OrientationError("AEXORI007", "repository state could not be inspected") from exc
    if stat.S_ISREG(mode):
        return "file"
    if stat.S_ISDIR(mode):
        return "directory"
    if stat.S_ISLNK(mode):
        return "link"
    return "special"


def _hash_file(path: Path) -> str:
    digest = hashlib.sha256()
    try:
        with path.open("rb") as handle:
            while chunk := handle.read(1 << 20):
                digest.update(chunk)
    except OSError as exc:
        raise OrientationError("AEXORI007", "repository state could not be read") from exc
    return digest.hexdigest()


def _tree_state(root: Path, *, git_only: bool = False) -> TreeState:
    files: dict[str, str] = {}
    if git_only:
        git = root / ".git"
        candidates: list[tuple[str, Path]] = []
        if git.is_file() or git.is_symlink():
            candidates.append((".git", git))
        elif git.is_dir():
            for fixed in ("HEAD", "packed-refs"):
                candidate = git / fixed
                if candidate.exists() or candidate.is_symlink():
                    candidates.append((f".git/{fixed}", candidate))
            refs = git / "refs"
            if refs.is_dir() and not refs.is_symlink():
                for directory, names, filenames in os.walk(refs, followlinks=False):
                    names[:] = sorted(names)
                    for filename in sorted(filenames):
                        candidate = Path(directory) / filename
                        candidates.append((candidate.relative_to(root).as_posix(), candidate))
        for relative, path in candidates:
            kind = _entry_kind(path)
            if kind == "file":
                files[relative] = "file:" + _hash_file(path)
            elif kind == "link":
                try:
                    files[relative] = "link:" + hashlib.sha256(os.readlink(path).encode("utf-8")).hexdigest()
                except OSError as exc:
                    raise OrientationError("AEXORI007", "Git reference state could not be read") from exc
            else:
                files[relative] = kind
    else:
        for directory, names, filenames in os.walk(root, followlinks=False):
            current = Path(directory)
            if current == root:
                names[:] = [name for name in sorted(names) if name != ".git"]
            else:
                names[:] = sorted(names)
            for name in list(names):
                candidate = current / name
                if candidate.is_symlink():
                    relative = candidate.relative_to(root).as_posix()
                    try:
                        files[relative] = "link:" + hashlib.sha256(os.readlink(candidate).encode("utf-8")).hexdigest()
                    except OSError as exc:
                        raise OrientationError("AEXORI007", "repository link state could not be read") from exc
                    names.remove(name)
            for filename in sorted(filenames):
                candidate = current / filename
                relative = candidate.relative_to(root).as_posix()
                kind = _entry_kind(candidate)
                if kind == "file":
                    files[relative] = "file:" + _hash_file(candidate)
                elif kind == "link":
                    try:
                        files[relative] = "link:" + hashlib.sha256(os.readlink(candidate).encode("utf-8")).hexdigest()
                    except OSError as exc:
                        raise OrientationError("AEXORI007", "repository link state could not be read") from exc
                else:
                    files[relative] = kind
                if len(files) > MAX_STATE_FILES:
                    raise OrientationError("AEXORI008", "repository exceeds the bounded orientation file count")
    manifest = [{"path": path, "state": files[path]} for path in sorted(files, key=lambda item: item.encode("utf-8"))]
    return TreeState(hashlib.sha256(_json_bytes(manifest)).hexdigest(), files)


def _changed_paths(before: TreeState, after: TreeState) -> list[str]:
    return sorted(
        {
            path
            for path in before.files.keys() | after.files.keys()
            if before.files.get(path) != after.files.get(path)
        },
        key=lambda item: item.encode("utf-8"),
    )


def _skill_manifest(core: Path) -> tuple[str, str]:
    records: list[dict[str, str]] = []
    contract: dict[str, Any] | None = None
    for directory, names, filenames in os.walk(core, followlinks=False):
        names[:] = sorted(names)
        for name in names:
            if (Path(directory) / name).is_symlink():
                raise OrientationError("AEXORI009", "portable skill contains a linked directory")
        for filename in sorted(filenames):
            path = Path(directory) / filename
            if path.is_symlink() or not path.is_file():
                raise OrientationError("AEXORI009", "portable skill contains a non-regular file")
            relative = path.relative_to(core).as_posix()
            try:
                text = path.read_text(encoding="utf-8")
            except (OSError, UnicodeError) as exc:
                raise OrientationError("AEXORI009", "portable skill contains unreadable non-UTF-8 content") from exc
            canonical = text.replace("\r\n", "\n").replace("\r", "\n").encode("utf-8")
            records.append({"mode": TEXT_MODE, "path": relative, "sha256": hashlib.sha256(canonical).hexdigest()})
            if relative == "skill-contract.json":
                contract = _json_object(canonical, "skill-contract")
    records.sort(key=lambda item: item["path"].encode("utf-8"))
    if not records or contract is None or not (core / "SKILL.md").is_file():
        raise OrientationError("AEXORI009", "portable skill is empty or incomplete")
    if (
        contract.get("schema") != CONTRACT_SCHEMA
        or contract.get("name") != "harness-orient"
        or contract.get("mutation_class") != "read-only"
        or contract.get("delegation") != {"allowed": False, "fallback": "single-agent"}
        or contract.get("evidence")
        != {"receipt_schema": RECEIPT_SCHEMA, "target_retention": False}
    ):
        raise OrientationError("AEXORI009", "portable skill contract does not declare the approved read-only boundary")
    manifest = {"files": records, "schema": MANIFEST_SCHEMA}
    manifest_bytes = _json_bytes(manifest)
    return str(contract.get("version")), hashlib.sha256(manifest_bytes).hexdigest()


def _lock_evaluator(target: Path) -> dict[str, str]:
    lock = target / ".engineering-harness.lock"
    if not lock.is_file() or lock.is_symlink():
        return {}
    try:
        value = json.loads(lock.read_text(encoding="utf-8"), object_pairs_hook=_reject_duplicate_keys)
    except (OSError, UnicodeError, json.JSONDecodeError, OrientationError, RecursionError):
        return {}
    if not isinstance(value, dict) or not isinstance(value.get("evaluator"), dict):
        return {}
    evaluator = value["evaluator"]
    result: dict[str, str] = {}
    for source, destination in (("payload_sha256", "payload"), ("archive_sha256", "wheel")):
        selected = evaluator.get(source)
        if isinstance(selected, str) and re.fullmatch(r"[0-9a-f]{64}", selected):
            result[destination] = selected
    return result


def _run(
    launcher: Sequence[str],
    arguments: Sequence[str],
    *,
    target: Path,
    operation: str,
    timeout: int = 120,
) -> CommandResult:
    environment = os.environ.copy()
    environment.pop("PYTHONPATH", None)
    environment["PYTHONNOUSERSITE"] = "1"
    try:
        completed = subprocess.run(
            [*launcher, *arguments],
            cwd=target,
            env=environment,
            stdin=subprocess.DEVNULL,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            check=False,
            shell=False,
            timeout=timeout,
        )
    except (OSError, subprocess.TimeoutExpired) as exc:
        detail = "could not be launched" if isinstance(exc, OSError) else "timed out"
        return CommandResult(operation, 127 if isinstance(exc, OSError) else 124, b"", detail.encode("ascii"), "failed")
    oversized = len(completed.stdout) > MAX_OUTPUT_BYTES or len(completed.stderr) > MAX_OUTPUT_BYTES
    return CommandResult(
        operation,
        completed.returncode if not oversized else 125,
        completed.stdout[:MAX_OUTPUT_BYTES],
        completed.stderr[:MAX_OUTPUT_BYTES],
        "passed" if completed.returncode == 0 and not oversized else "failed",
    )


def _diagnostics(result: CommandResult, replacements: Sequence[str]) -> list[str]:
    raw = result.stderr if result.stderr.strip() else result.stdout
    text = raw.decode("utf-8", "replace")
    for replacement in sorted({item for item in replacements if item}, key=len, reverse=True):
        text = text.replace(replacement, "<redacted-path>")
    text = SECRET_ASSIGNMENT.sub(lambda matched: f"{matched.group(1)}=<redacted>", text)
    lines = [line.strip()[:512] for line in text.splitlines() if line.strip()]
    return lines[:20]


def _operation_failure(result: CommandResult, replacements: Sequence[str]) -> dict[str, Any]:
    return {
        "code": "AEXORI022",
        "diagnostics": _diagnostics(result, replacements),
        "operation": result.operation,
    }


def _command_supported(help_text: str, command: str) -> bool:
    return re.search(rf"(?m)^\s*{re.escape(command)}\s+", help_text) is not None or command in help_text


def _focus_projection(focus: Mapping[str, Any] | None, artifact: str | None) -> dict[str, Any]:
    if artifact is None:
        return {"artifact": None, "status": "not_requested", "scope": {}}
    if focus is None:
        return {"artifact": artifact, "status": "not_assessable", "scope": {}}
    state = focus.get("state") if isinstance(focus.get("state"), dict) else {}
    before = state.get("before") if isinstance(state.get("before"), list) else []
    lifecycle = None
    for item in before:
        if isinstance(item, dict) and item.get("id") == artifact and isinstance(item.get("status"), str):
            lifecycle = item["status"]
            break
    scope = focus.get("scope") if isinstance(focus.get("scope"), dict) else {}
    return {
        "artifact": artifact,
        "lifecycle_state": lifecycle,
        "status": "available",
        "scope": {
            "declared_paths": scope.get("declared_paths", []),
            "dependencies": scope.get("dependencies", []),
            "governing": scope.get("governing", []),
        },
    }


def _decision_projection(focus: Mapping[str, Any] | None, inspection: Mapping[str, Any] | None) -> dict[str, Any]:
    if focus is not None:
        restitution = focus.get("restitution") if isinstance(focus.get("restitution"), dict) else {}
        decision = restitution.get("decision_required") if isinstance(restitution.get("decision_required"), dict) else None
        next_value = restitution.get("next") if isinstance(restitution.get("next"), dict) else {}
        return {
            "command_or_suggested_response": restitution.get("command_or_response"),
            "recommendation": next_value.get("action") or "Review the selected harness result.",
            "required_accountable_role": decision.get("role") if decision else None,
        }
    queues = inspection.get("queues") if isinstance(inspection, dict) and isinstance(inspection.get("queues"), dict) else {}
    for queue_name in ("decision_required", "definition_pending", "assurance_pending", "active_work"):
        items = queues.get(queue_name)
        if not isinstance(items, list) or not items or not isinstance(items[0], dict):
            continue
        selected = items[0]
        owners = selected.get("owners") if isinstance(selected.get("owners"), list) else []
        action = selected.get("action") if isinstance(selected.get("action"), str) else "review-repository-state"
        return {
            "command_or_suggested_response": {"kind": "suggested-response", "value": action},
            "recommendation": action,
            "required_accountable_role": owners[0] if owners else None,
        }
    return {
        "command_or_suggested_response": None,
        "recommendation": "No accountable decision is currently queued.",
        "required_accountable_role": None,
    }


def _findings_projection(focus: Mapping[str, Any] | None) -> tuple[list[Any], list[Any]]:
    if focus is None or not isinstance(focus.get("findings"), dict):
        return [], []
    findings = focus["findings"]
    selected = findings.get("scoped_blockers") if isinstance(findings.get("scoped_blockers"), list) else []
    repository = findings.get("repository_blockers") if isinstance(findings.get("repository_blockers"), list) else []
    return selected, repository


def _repository_name(target: Path, inspection: Mapping[str, Any] | None) -> str:
    repository = inspection.get("repository") if isinstance(inspection, dict) and isinstance(inspection.get("repository"), dict) else {}
    name = repository.get("name")
    if isinstance(name, str) and name and len(name) <= 256:
        return name
    return target.name[:256]


def run_orientation(args: argparse.Namespace) -> tuple[dict[str, Any], int]:
    target = _safe_target(args.target)
    launcher = _parse_launcher(args.evaluator_launcher_json)
    expected_version = args.expected_evaluator_version
    expected_tuple = _version_tuple(expected_version)
    if expected_tuple < MINIMUM_VERSION:
        raise OrientationError("AEXORI010", "evaluators older than 0.5.0 are unsupported", outcome="blocked")
    expected_root = str(Path(args.expected_evaluator_root).expanduser().resolve())
    artifact = args.artifact
    if artifact is not None and ARTIFACT_ID.fullmatch(artifact) is None:
        raise OrientationError("AEXORI011", "selected artifact has an invalid identifier", outcome="blocked")
    if args.preflight_phase is not None and (artifact is None or not artifact.startswith("WO-")):
        raise OrientationError("AEXORI012", "preflight requires an explicitly selected work order", outcome="blocked")

    core = Path(__file__).resolve().parents[1]
    skill_version, skill_digest = _skill_manifest(core)
    repository_before = _tree_state(target)
    git_before = _tree_state(target, git_only=True)
    operations: list[CommandResult] = []
    deviations: list[dict[str, Any]] = []
    residual = [
        "Read-only observations cannot prove that undisclosed work did not occur outside the measured repository and Git-reference boundary."
    ]
    outcome = "completed"
    evaluator_version: str | None = None
    identity = "not_assessed"
    integrity: dict[str, Any] = {"diagnostics": [], "outcome": "not_assessed"}
    validation: dict[str, Any] = {"outcome": "not_assessed"}
    inspection: dict[str, Any] | None = None
    focus: dict[str, Any] | None = None
    preflight: dict[str, Any] | None = None
    replacements = [str(target), expected_root, launcher[0]]

    version_result = _run(launcher, ["--version"], target=target, operation="version")
    operations.append(version_result)
    if version_result.returncode != 0:
        outcome = "blocked"
        deviations.append(_operation_failure(version_result, replacements))
    else:
        evaluator_version = version_result.stdout.decode("utf-8", "replace").strip()
        if evaluator_version != expected_version or _version_tuple(evaluator_version) < MINIMUM_VERSION:
            outcome = "blocked"
            deviations.append(
                {
                    "code": "AEXORI013",
                    "expected": expected_version,
                    "observed": evaluator_version,
                    "operation": "version",
                }
            )

    if outcome == "completed":
        identity_arguments = [
            "identity",
            "--role",
            "released-evaluator",
            "--expected-version",
            expected_version,
            "--expected-root",
            expected_root,
            "--checkout-root",
            str(target),
            "--require-isolated-python",
        ]
        lock_identity = _lock_evaluator(target)
        if "payload" in lock_identity:
            identity_arguments.extend(["--evaluator-payload-sha256", lock_identity["payload"]])
        if "wheel" in lock_identity:
            identity_arguments.extend(["--evaluator-wheel-sha256", lock_identity["wheel"]])
        identity_result = _run(launcher, identity_arguments, target=target, operation="identity")
        operations.append(identity_result)
        identity = "passed" if identity_result.returncode == 0 else "failed"
        if identity_result.returncode != 0:
            outcome = "blocked"
            deviations.append(_operation_failure(identity_result, replacements))

    help_text = ""
    if outcome == "completed":
        help_result = _run(launcher, ["--help"], target=target, operation="capability-help")
        operations.append(help_result)
        if help_result.returncode != 0:
            outcome = "blocked"
            deviations.append(_operation_failure(help_result, replacements))
        else:
            help_text = help_result.stdout.decode("utf-8", "replace")

    if outcome == "completed":
        doctor_result = _run(launcher, ["doctor", str(target)], target=target, operation="doctor")
        operations.append(doctor_result)
        integrity = {
            "diagnostics": [] if doctor_result.returncode == 0 else _diagnostics(doctor_result, replacements),
            "outcome": "passed" if doctor_result.returncode == 0 else "failed",
        }
        if doctor_result.returncode != 0:
            outcome = "blocked"
            deviations.append(_operation_failure(doctor_result, replacements))

    if outcome == "completed":
        validate_result = _run(launcher, ["validate", str(target), "--json"], target=target, operation="validate-json")
        operations.append(validate_result)
        try:
            validation_report = _json_object(validate_result.stdout, "validate-json")
        except OrientationError as exc:
            outcome = "failed"
            deviations.append({"code": exc.code, "message": exc.message, "operation": "validate-json"})
        else:
            valid = validation_report.get("valid")
            if not isinstance(valid, bool):
                outcome = "failed"
                deviations.append({"code": "AEXORI021", "message": "validation JSON has no boolean valid field", "operation": "validate-json"})
            else:
                validation = {
                    "error_count": validation_report.get("error_count", len(validation_report.get("errors", [])) if isinstance(validation_report.get("errors"), list) else 0),
                    "outcome": "passed" if valid and validate_result.returncode == 0 else "blocked",
                    "valid": valid,
                    "warning_count": validation_report.get("warning_count", len(validation_report.get("warnings", [])) if isinstance(validation_report.get("warnings"), list) else 0),
                }
                if not valid or validate_result.returncode != 0:
                    outcome = "blocked"
                    deviations.append(
                        {
                            "code": "AEXORI023",
                            "errors": validation_report.get("errors", []),
                            "operation": "validate-json",
                        }
                    )

    if outcome == "completed":
        inspect_result = _run(launcher, ["inspect", str(target), "--json"], target=target, operation="inspect-json")
        operations.append(inspect_result)
        if inspect_result.returncode != 0:
            outcome = "blocked"
            deviations.append(_operation_failure(inspect_result, replacements))
        else:
            try:
                inspection = _json_object(inspect_result.stdout, "inspect-json")
            except OrientationError as exc:
                outcome = "failed"
                deviations.append({"code": exc.code, "message": exc.message, "operation": "inspect-json"})

    if outcome == "completed" and artifact is not None:
        if not _command_supported(help_text, "focus"):
            outcome = "degraded"
            deviations.append({"code": "AEXORI030", "operation": "focus-json", "status": "not_assessable"})
        else:
            focus_help = _run(launcher, ["focus", "--help"], target=target, operation="focus-help")
            operations.append(focus_help)
            focus_help_text = focus_help.stdout.decode("utf-8", "replace")
            if focus_help.returncode != 0 or "--json" not in focus_help_text or "--artifact" not in focus_help_text:
                outcome = "degraded"
                deviations.append({"code": "AEXORI030", "operation": "focus-json", "status": "not_assessable"})
            else:
                focus_arguments = ["focus", str(target), "--artifact", artifact, "--json"]
                if "--result-schema" in focus_help_text:
                    focus_arguments.extend(["--result-schema", "2"])
                focus_result = _run(launcher, focus_arguments, target=target, operation="focus-json")
                operations.append(focus_result)
                if focus_result.returncode != 0:
                    outcome = "blocked"
                    deviations.append(_operation_failure(focus_result, replacements))
                else:
                    try:
                        focus = _json_object(focus_result.stdout, "focus-json")
                    except OrientationError as exc:
                        outcome = "failed"
                        deviations.append({"code": exc.code, "message": exc.message, "operation": "focus-json"})

    if outcome in {"completed", "degraded"} and args.preflight_phase is not None:
        if not _command_supported(help_text, "preflight"):
            outcome = "degraded"
            deviations.append({"code": "AEXORI031", "operation": "preflight", "status": "not_assessable"})
        else:
            preflight_result = _run(
                launcher,
                [
                    "preflight",
                    str(target),
                    "--work-order",
                    artifact,
                    "--phase",
                    args.preflight_phase,
                    "--json",
                ],
                target=target,
                operation="preflight",
            )
            operations.append(preflight_result)
            try:
                preflight = _json_object(preflight_result.stdout, "preflight")
            except OrientationError as exc:
                outcome = "failed"
                deviations.append({"code": exc.code, "message": exc.message, "operation": "preflight"})
            else:
                ready = preflight.get("ready")
                if not isinstance(ready, bool):
                    outcome = "failed"
                    deviations.append({"code": "AEXORI021", "message": "preflight JSON has no boolean ready field", "operation": "preflight"})
                elif not ready or preflight_result.returncode != 0:
                    outcome = "blocked"
                    deviations.append({"code": "AEXORI032", "operation": "preflight", "ready": ready})

    repository_after = _tree_state(target)
    git_after = _tree_state(target, git_only=True)
    changed_paths = _changed_paths(repository_before, repository_after) + _changed_paths(git_before, git_after)
    changed_paths = sorted(set(changed_paths), key=lambda item: item.encode("utf-8"))
    if changed_paths:
        outcome = "failed"
        deviations.append({"code": "AEXORI040", "changed_paths": changed_paths})

    repository_name = _repository_name(target, inspection)
    selected_blockers, repository_blockers = _findings_projection(focus)
    inspection_summary = inspection.get("summary") if isinstance(inspection, dict) and isinstance(inspection.get("summary"), dict) else {}
    focus_findings = focus.get("findings") if isinstance(focus, dict) and isinstance(focus.get("findings"), dict) else {}
    background_count = focus_findings.get("unrelated_count")
    if not isinstance(background_count, int):
        background_count = inspection_summary.get("finding_count", 0) if isinstance(inspection_summary.get("finding_count", 0), int) else 0
    decision = _decision_projection(focus, inspection)
    receipt = {
        "effects": {
            "changed_paths": changed_paths,
            "evidence": [{"kind": "portable-skill-manifest", "sha256": skill_digest}],
            "state_after": [
                {"kind": "repository-bytes", "sha256": repository_after.digest},
                {"kind": "git-references", "sha256": git_after.digest},
            ],
            "state_before": [
                {"kind": "repository-bytes", "sha256": repository_before.digest},
                {"kind": "git-references", "sha256": git_before.digest},
            ],
        },
        "execution": {
            "operations": [result.receipt_entry() for result in operations],
            "profiles": ["single-agent-orientation"],
            "skills": [{"name": "harness-orient", "portable_core_sha256": skill_digest, "version": skill_version}],
            "worker_results": [],
        },
        "schema": RECEIPT_SCHEMA,
        "selection": {
            "artifact": artifact,
            "autonomy_envelope_sha256": None,
            "repository": repository_name,
        },
        "validation": {
            "deviations": deviations,
            "evaluator": {"identity": identity, "version": evaluator_version},
            "gates": [],
            "outcome": outcome,
            "residual_uncertainty": residual,
        },
    }
    receipt_digest = hashlib.sha256(_json_bytes(receipt)).hexdigest()
    orientation = {
        "background_observation_count": background_count,
        "blockers": {"repository": repository_blockers, "selected": selected_blockers},
        "candidate_source": {"governing": False, "status": "not_assessed"},
        "decision": decision,
        "execution_receipt": receipt,
        "execution_receipt_sha256": receipt_digest,
        "inspection": {
            "queues": inspection.get("queues", {}) if isinstance(inspection, dict) else {},
            "summary": inspection_summary,
        },
        "integrity": integrity,
        "non_effects": [
            "No repository or Git mutation was requested.",
            "No lifecycle transition or accountable decision was applied.",
            "No evaluator installation, network, credential, or external action was performed.",
            "No receipt or evidence file was retained in the target.",
        ],
        "outcome": outcome,
        "preflight": preflight if preflight is not None else {"status": "not_requested"},
        "released_evaluator": {"identity": identity, "version": evaluator_version},
        "repository": {"name": repository_name, "root": "."},
        "schema": ORIENTATION_SCHEMA,
        "selected": _focus_projection(focus, artifact),
        "validation": validation,
    }
    return orientation, 0 if outcome in {"completed", "degraded"} else 2


def _failed_result(exc: OrientationError, args: argparse.Namespace) -> dict[str, Any]:
    artifact = args.artifact if isinstance(getattr(args, "artifact", None), str) else None
    receipt = {
        "effects": {"changed_paths": [], "evidence": [], "state_after": [], "state_before": []},
        "execution": {"operations": [], "profiles": ["single-agent-orientation"], "skills": [], "worker_results": []},
        "schema": RECEIPT_SCHEMA,
        "selection": {"artifact": artifact, "autonomy_envelope_sha256": None, "repository": None},
        "validation": {
            "deviations": [{"code": exc.code, "message": exc.message}],
            "evaluator": {},
            "gates": [],
            "outcome": exc.outcome,
            "residual_uncertainty": [],
        },
    }
    receipt_digest = hashlib.sha256(_json_bytes(receipt)).hexdigest()
    return {
        "background_observation_count": 0,
        "blockers": {"repository": [], "selected": []},
        "candidate_source": {"governing": False, "status": "not_assessed"},
        "decision": {
            "command_or_suggested_response": None,
            "recommendation": "Resolve the reported orientation prerequisite, then retry.",
            "required_accountable_role": None,
        },
        "execution_receipt": receipt,
        "execution_receipt_sha256": receipt_digest,
        "inspection": {"queues": {}, "summary": {}},
        "integrity": {"diagnostics": [], "outcome": "not_assessed"},
        "non_effects": ["No repository, Git, lifecycle, evaluator, network, credential, or external mutation was performed."],
        "outcome": exc.outcome,
        "preflight": {"status": "not_requested"},
        "released_evaluator": {"identity": "not_assessed", "version": None},
        "repository": {"name": None, "root": "."},
        "schema": ORIENTATION_SCHEMA,
        "selected": {"artifact": artifact, "status": "not_assessable", "scope": {}},
        "validation": {"outcome": "not_assessed"},
    }


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="Run the read-only harness-orient procedure.")
    parser.add_argument("target")
    parser.add_argument("--evaluator-launcher-json", required=True)
    parser.add_argument("--expected-evaluator-version", required=True)
    parser.add_argument("--expected-evaluator-root", required=True)
    parser.add_argument("--artifact")
    parser.add_argument("--preflight-phase", choices=("start", "review"))
    return parser


def main(argv: Sequence[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    try:
        result, returncode = run_orientation(args)
    except OrientationError as exc:
        result = _failed_result(exc, args)
        returncode = 2
    sys.stdout.buffer.write(_json_bytes(result))
    return returncode


if __name__ == "__main__":
    raise SystemExit(main())
