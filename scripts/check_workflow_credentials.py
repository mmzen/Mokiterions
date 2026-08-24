#!/usr/bin/env python3
"""Refuse a workflow definition that could spend money at a model provider.

Repository-owned, absent from `.engineering-harness.lock`, decides nothing, writes
nothing, reaches no network and reads no credential of its own.

This implements `REQ-MOK-073` and `SPEC-MOK-007` rule 13.6. The requirement's own
rationale says why it is a static check over the definitions rather than a test over the
runs: the containment that actually holds is capability-based, so what has to be
established is that no workflow *asks* for a provider credential, on a machine where
none exists. A runtime check could only report on the runs that already happened, and by
then the interesting case has already spent money.

Five scans: one per clause of the requirement's *Required response* item 2, one for its
live-mode prohibition, and one for item 3, which is the only one of them that requires a
workflow to do something rather than to refrain from it.

  A. **Not as a secret.** Any `secrets.<name>` reference outside a small allowlist
     refuses. This is deliberately a *closed* list rather than a provider-name pattern:
     a credential named `SPARKLE_JUICE` is still spendable, and the containment is that
     the repository's Actions secrets hold nothing but the platform's own token. A
     future secret that genuinely belongs here needs an edit to `PERMITTED_SECRETS`,
     which is an accountable line in a diff rather than a silent pass.
  B. **Not as an environment variable and not as an input.** Two patterns. Any
     model-provider vocabulary, in any case, anywhere on a line; and any
     environment-variable-shaped name whose spelling says "credential" — matched only in
     upper case, so that `persist-credentials: false` on a checkout step is not a finding
     while `PROVIDER_API_KEY` is.
  C. **Not through a step that fetches one.** A step that brokers a secret from a cloud
     vault or a secret manager acquires a credential without naming it, which defeats
     scans A and B entirely. Both the action references and the command-line spellings
     are listed.
  D. **No live-mode selection, and no selection this check cannot see the transcript of.**
     The options that select a live run arrive with `WO-MOK-026` and their spellings are
     not fixed by any approved artifact yet, so the list below is the vocabulary those
     options will plausibly use and is not a specification of them. The load-bearing half
     of this scan needs no option name: a workflow that selects the model-backed source
     must, on the same line, name a transcript that is a file in this repository. That is
     `REQ-MOK-073` *Required response* item 3 read as a property of a definition — a
     selection whose transcript this check cannot see is a selection that may reach a
     provider, and so is a selection whose named transcript is not committed.
  E. **The source is still exercised.** Clauses A to D are all satisfied by a repository
     whose automation never runs the engine at all, which would make this check green and
     `REQ-MOK-073` vacuous. So at least one definition must select the source in replay
     mode against a committed transcript. This is `VER-MOK-018` case **L21b**, checked
     rather than assumed, and it is the only clause here that requires a workflow to do
     something rather than to refrain from it.

What this program deliberately does NOT do:

  * decide whether the credential exists in the repository's Actions secrets. It cannot
    see them. `REQ-MOK-073`'s *Constraints* records their absence as the condition the
    whole containment rests on, and `VER-MOK-018` case **L21a** carries it as an owner
    attestation. A pass here means no workflow asks; it does not mean no secret exists.
  * read the transcript, or replay it. Clause E establishes that a step selects the source
    against a file that exists; whether that file is the recording it claims to be is
    `mokiterions-core/tests/replay.rs`, which compares it byte for byte against the run
    recorded in-process, and whether the replay reproduces that run is the same file's
    binary comparison. This check would have to build the engine to establish either.
  * approve anything. It can refuse a merge and it approves nothing.

It fails closed. A missing directory, a directory holding no definition, and a file it
cannot decode are all refusals, because a scan that examined nothing must not report
that it found nothing.

Comment lines are not scanned, and that is a property rather than a gap: a comment is
resolved by no runner, so a credential named in one cannot be spent. It also lets this
repository write down the prohibition, in a workflow, without the prohibition tripping
over its own statement.

Usage:
    python scripts/check_workflow_credentials.py --root .
"""

from __future__ import annotations

import argparse
import re
import sys
from dataclasses import dataclass
from pathlib import Path

# Every YAML definition the platform reads, not only the workflow files. `SPEC-MOK-007`
# rule 13.6's subject is a workflow file; a composite action under `.github/actions` is a
# step, and a step that fetches a credential is exactly what clause C is about. Scanning
# the wider set can only refuse more than the rule requires, never less.
DEFINITION_ROOT = Path(".github")
DEFINITION_SUFFIXES = (".yml", ".yaml")

# The platform's own scoped token, under the two spellings this repository uses. It is
# issued per job by the runner, is not a model-provider credential, and cannot be spent
# at one. `release.yml` passes `github.token` to a single publish step under `GH_TOKEN`.
PERMITTED_SECRETS = frozenset({"GITHUB_TOKEN"})
PERMITTED_ENVIRONMENT_NAMES = frozenset({"GITHUB_TOKEN", "GH_TOKEN"})

SECRET_PROPERTY = re.compile(r"\bsecrets\s*\.\s*([A-Za-z_][A-Za-z0-9_-]*)")
SECRET_SUBSCRIPT = re.compile(r"""\bsecrets\s*\[\s*['"]([^'"]+)['"]\s*\]""")
# `toJSON(secrets)` and a bare `${{ secrets }}` hand over the whole context, so no name
# appears to compare against the allowlist. Both are refused on sight.
SECRET_WHOLESALE = re.compile(r"toJSON\s*\(\s*secrets\s*\)|\$\{\{\s*secrets\s*\}\}")

PROVIDER_VOCABULARY = (
    "openai",
    "anthropic",
    "azure_openai",
    "azureopenai",
    "openrouter",
    "bedrock",
    "vertexai",
    "gemini",
    "mistral",
    "cohere",
    "huggingface",
    "replicate",
    "fireworks",
    "together_ai",
    "perplexity",
    "deepseek",
    "groq",
    "ollama",
)
PROVIDER_PATTERN = re.compile("|".join(re.escape(name) for name in PROVIDER_VOCABULARY))

# Upper case only. A credential's environment name is conventionally upper case, and
# restricting the match to that case is what keeps `persist-credentials`, a checkout
# input with no credential in it, out of the findings.
ENVIRONMENT_NAME = re.compile(r"\b[A-Z][A-Z0-9]*(?:_[A-Z0-9]+)*\b")
CREDENTIAL_WORDS = (
    "API_KEY",
    "APIKEY",
    "ACCESS_KEY",
    "PRIVATE_KEY",
    "SECRET",
    "TOKEN",
    "CREDENTIAL",
    "PASSWORD",
    "PASSWD",
    "BEARER",
)

CREDENTIAL_BROKER_ACTIONS = (
    "azure/login",
    "aws-actions/configure-aws-credentials",
    "aws-actions/aws-secretsmanager-get-secrets",
    "google-github-actions/auth",
    "hashicorp/vault-action",
    "1password/load-secrets-action",
    "bitwarden/sm-action",
    "akeyless-community/action-get-secrets",
    "dopplerhq/cli-action",
    "infisical/secrets-action",
)
CREDENTIAL_BROKER_COMMANDS = (
    "op://",
    "op read",
    "vault read",
    "vault kv get",
    "aws secretsmanager",
    "az keyvault",
    "gcloud secrets",
    "doppler secrets",
    "sops -d",
    "sops --decrypt",
)

# `WO-MOK-026`'s vocabulary, listed to be refused and not to be specified. See scan D in
# the module docstring: the option names are not fixed by an approved artifact yet.
LIVE_MODE_OPTIONS = (
    "--live",
    "--live-mode",
    "--no-replay",
    "--allow-spend",
    "--real-run",
    "--spend-ceiling",
    "--connector",
    "--connector-path",
)
LIVE_MODE_PATTERN = re.compile(
    r"(?<![\w-])(?:" + "|".join(re.escape(option) for option in LIVE_MODE_OPTIONS) + r")(?![\w-])"
)

# The model-backed source's name, as `SPEC-MOK-007` rule 18.1 fixes it and as
# `mokiterions-core/src/simulation.rs` writes it. Both an option spelling and a YAML
# mapping value are matched, because a workflow can reach the engine either way.
MODEL_BACKED_SOURCE = "llm"
SOURCE_SELECTION = re.compile(
    r"--policy[=\s]+" + MODEL_BACKED_SOURCE + r"\b|\bpolicy\s*:\s*['\"]?" + MODEL_BACKED_SOURCE + r"\b"
)
# The transcript, as an option on the *same line* as the selection. One line, because this
# check reads one line at a time and joins none: a workflow that spread the command across
# a block scalar would state the selection on one line and its transcript on another, and
# this would refuse it. That is a real constraint on how the replay step is written, and
# the step in `provider-credentials.yml` says so in a comment beside itself.
TRANSCRIPT_ARGUMENT = re.compile(r"--transcript-path[=\s]+([^\s'\"]+)")

# Flipped from `False` by `WO-MOK-025` item 11's second half, in the commit that committed
# `mokiterions-core/tests/transcript-seed0-ticks20-hunting.jsonl` and added the replay step
# to `provider-credentials.yml`. It is kept rather than deleted because it is the line that
# records the relaxation and its direction: `False` means no workflow may select the source
# at all, because there is nothing to replay; `True` means a workflow may select it only in
# replay mode against a transcript this repository holds, and at least one workflow must.
REPLAY_TRANSCRIPT_COMMITTED = True


class Refusal(Exception):
    """A reason to refuse the scan itself, distinct from a finding within it."""


@dataclass(frozen=True)
class Finding:
    """One reference, at one line, with the clause it violates."""

    path: str
    line_number: int
    text: str
    clause: str
    reason: str


@dataclass(frozen=True)
class ReplayStep:
    """One permitted selection of the model-backed source, and what it replays.

    The counterpart of `Finding`: the same reading of the same line either refuses it or
    records it as clause E's evidence. It is a separate type rather than a `Finding` with
    an inverted meaning because it is reported on a pass, where a finding never is.
    """

    path: str
    line_number: int
    transcript: str


def definition_files(root: Path) -> list[Path]:
    """Every YAML definition the platform would read, in a stable order."""
    directory = root / DEFINITION_ROOT
    if not directory.is_dir():
        raise Refusal(
            f"{DEFINITION_ROOT.as_posix()} is not a directory under {root}. This check "
            "establishes a property of the repository's automation definitions; with no "
            "definitions to read it establishes nothing, and reporting success would say "
            "that no workflow asks for a credential when in fact none was examined."
        )

    found = sorted(
        path
        for path in directory.rglob("*")
        if path.is_file() and path.suffix in DEFINITION_SUFFIXES
    )
    if not found:
        raise Refusal(
            f"{DEFINITION_ROOT.as_posix()} holds no {' or '.join(DEFINITION_SUFFIXES)} "
            "file. A scan with nothing in it is a refusal rather than a pass, for the "
            "same reason a missing directory is."
        )
    return found


def readable_lines(path: Path, display: str) -> list[str]:
    try:
        text = path.read_text(encoding="utf-8")
    except UnicodeDecodeError as exc:
        raise Refusal(
            f"{display} is not valid UTF-8 and cannot be scanned: {exc}. A definition "
            "this check cannot read is a refusal, not a file to skip."
        ) from exc
    except OSError as exc:
        raise Refusal(f"cannot read {display}: {exc}") from exc
    return text.splitlines()


def is_comment(line: str) -> bool:
    """A line the platform resolves nothing on.

    Only a line whose first non-space character is `#` counts, which is conservative: a
    trailing `#` in YAML also begins a comment, and inside a `run:` block scalar it
    begins a shell comment, but neither is excluded here. Scanning those is a refusal the
    requirement does not ask for and never a reference it misses.
    """
    return line.lstrip().startswith("#")


def scan_secret_references(display: str, number: int, line: str) -> list[Finding]:
    findings: list[Finding] = []

    if SECRET_WHOLESALE.search(line):
        findings.append(
            Finding(
                display,
                number,
                line.strip(),
                "A: not as a secret",
                "the whole secrets context is handed over, so no name is even stated; "
                "every secret the repository holds reaches this step",
            )
        )

    for pattern in (SECRET_PROPERTY, SECRET_SUBSCRIPT):
        for match in pattern.finditer(line):
            name = match.group(1)
            if name.upper() in PERMITTED_SECRETS:
                continue
            findings.append(
                Finding(
                    display,
                    number,
                    line.strip(),
                    "A: not as a secret",
                    f"references the secret {name}, which is not one of the platform's "
                    f"own scoped tokens ({', '.join(sorted(PERMITTED_SECRETS))}). "
                    "REQ-MOK-073 keeps every spendable credential out of automation, and "
                    "the allowlist is closed because a credential is spendable whatever "
                    "it is called",
                )
            )
    return findings


def scan_names(display: str, number: int, line: str) -> list[Finding]:
    findings: list[Finding] = []

    lowered = line.lower()
    provider = PROVIDER_PATTERN.search(lowered)
    if provider is not None:
        findings.append(
            Finding(
                display,
                number,
                line.strip(),
                "B: not as an environment variable and not as an input",
                f"names the model provider {provider.group(0)!r}. No workflow in this "
                "repository reaches a provider, so no workflow has a reason to name one",
            )
        )

    for match in ENVIRONMENT_NAME.finditer(line):
        name = match.group(0)
        if name in PERMITTED_ENVIRONMENT_NAMES:
            continue
        word = next((word for word in CREDENTIAL_WORDS if word in name), None)
        if word is None:
            continue
        findings.append(
            Finding(
                display,
                number,
                line.strip(),
                "B: not as an environment variable and not as an input",
                f"the name {name} reads as a credential ({word}). Only the platform's "
                f"own token is permitted here ({', '.join(sorted(PERMITTED_ENVIRONMENT_NAMES))})",
            )
        )
    return findings


def scan_broker_steps(display: str, number: int, line: str) -> list[Finding]:
    findings: list[Finding] = []
    lowered = line.lower()

    for action in CREDENTIAL_BROKER_ACTIONS:
        if action in lowered:
            findings.append(
                Finding(
                    display,
                    number,
                    line.strip(),
                    "C: not through a step that fetches one",
                    f"the step uses {action}, which obtains a credential without naming "
                    "it, so scans A and B would both pass while the job held one",
                )
            )

    for command in CREDENTIAL_BROKER_COMMANDS:
        if command in lowered:
            findings.append(
                Finding(
                    display,
                    number,
                    line.strip(),
                    "C: not through a step that fetches one",
                    f"the step runs {command!r}, which reads a secret from a manager "
                    "outside the repository",
                )
            )
    return findings


def scan_live_mode(display: str, number: int, line: str) -> list[Finding]:
    findings: list[Finding] = []

    option = LIVE_MODE_PATTERN.search(line)
    if option is not None:
        findings.append(
            Finding(
                display,
                number,
                line.strip(),
                "D: no live-mode selection",
                f"{option.group(0)} selects or supplies a live run. A workflow that asks "
                "to spend is one secret away from spending, which REQ-MOK-073's *Failure "
                "and boundary behavior* refuses even with no credential present",
            )
        )

    return findings


def scan_source_selection(
    root: Path, display: str, number: int, line: str
) -> tuple[list[Finding], ReplayStep | None]:
    """Clause D's load-bearing half and clause E's evidence, which are one reading.

    Unlike the four scans above this one needs the root, because the question it asks is
    not about the line's vocabulary but about whether the file the line names is in the
    repository. A selection is either refused or returned as a replay step; it is never
    both and never neither.
    """
    if SOURCE_SELECTION.search(line) is None:
        return [], None

    def refused(reason: str) -> tuple[list[Finding], None]:
        return [Finding(display, number, line.strip(), "D: no live-mode selection", reason)], None

    if not REPLAY_TRANSCRIPT_COMMITTED:
        return refused(
            f"selects the {MODEL_BACKED_SOURCE!r} source. REQ-MOK-073 permits this in "
            "replay mode against a transcript committed to the repository, and no such "
            "transcript is committed yet, so there is no way for a workflow to select "
            "this source without asking to reach a provider"
        )

    named = TRANSCRIPT_ARGUMENT.search(line)
    if named is None:
        return refused(
            f"selects the {MODEL_BACKED_SOURCE!r} source and names no transcript on the "
            "same line. REQ-MOK-073 item 3 permits automation to exercise this source only "
            "in replay mode against a committed transcript, and a selection whose "
            "transcript this check cannot see is a selection that may reach a provider"
        )

    transcript = named.group(1)
    if not (root / transcript).is_file():
        return refused(
            f"replays {transcript}, which is not a file in this repository. A replay needs "
            "the transcript to be committed — REQ-MOK-073 item 3 says so in those words — "
            "and a step pointed at a path that is not there fails at the first opportunity "
            "or, worse, is corrected by pointing it at a provider"
        )

    return [], ReplayStep(display, number, transcript)


SCANS = (scan_secret_references, scan_names, scan_broker_steps, scan_live_mode)


def replay_step_missing(replay_steps: list[ReplayStep]) -> bool:
    """Clause E as one predicate, so the verdict and the report cannot disagree."""
    return REPLAY_TRANSCRIPT_COMMITTED and not replay_steps


def check(root: Path) -> tuple[list[Path], int, list[Finding], list[ReplayStep]]:
    """Every definition, every scan. Returns what was read and what was found."""
    files = definition_files(root)
    findings: list[Finding] = []
    replay_steps: list[ReplayStep] = []
    scanned_lines = 0

    for path in files:
        display = path.relative_to(root).as_posix()
        for number, line in enumerate(readable_lines(path, display), start=1):
            if is_comment(line):
                continue
            scanned_lines += 1
            for scan in SCANS:
                findings.extend(scan(display, number, line))
            refusals, replay_step = scan_source_selection(root, display, number, line)
            findings.extend(refusals)
            if replay_step is not None:
                replay_steps.append(replay_step)

    return files, scanned_lines, findings, replay_steps


def report(
    root: Path,
    files: list[Path],
    scanned_lines: int,
    findings: list[Finding],
    replay_steps: list[ReplayStep],
) -> None:
    for finding in findings:
        print(f"REFUSED: {finding.path}:{finding.line_number}", file=sys.stderr)
        print(f"  clause  {finding.clause}", file=sys.stderr)
        print(f"  line    {finding.text}", file=sys.stderr)
        print(f"  reason  {finding.reason}", file=sys.stderr)
        print(file=sys.stderr)

    if replay_step_missing(replay_steps):
        print("REFUSED: no definition exercises the model-backed source", file=sys.stderr)
        print("  clause  E: the source is still exercised", file=sys.stderr)
        print(
            "  reason  REQ-MOK-073 *Required response* item 3 requires a workflow step "
            "that exercises this\n          source in replay mode against a committed "
            "transcript, and VER-MOK-018 case L21b is\n          that requirement checked "
            "rather than assumed. Clauses A to D are all satisfied by a\n          "
            "repository whose automation never runs the engine, so a pass on them alone "
            "would say\n          nothing about a containment that only matters while the "
            "source is in use.",
            file=sys.stderr,
        )
        print(file=sys.stderr)

    print(f"Definitions under {DEFINITION_ROOT.as_posix()}, at {root}:")
    for path in files:
        print(f"  {path.relative_to(root).as_posix()}")
    print(f"  {len(files)} file(s), {scanned_lines} non-comment line(s) scanned")

    if findings or replay_step_missing(replay_steps):
        if findings:
            print(
                f"REFUSED: {len(findings)} credential or live-mode reference(s) in "
                f"{len({finding.path for finding in findings})} file(s)",
                file=sys.stderr,
            )
        return

    print("PASS: no workflow references a model-provider credential and none selects live mode")
    print("  scan A  no secret outside " + ", ".join(sorted(PERMITTED_SECRETS)))
    print("  scan B  no provider name and no credential-shaped environment name")
    print("  scan C  no step that brokers a secret from outside the repository")
    print("  scan D  no live-mode option, and every selection replays a committed transcript")
    print("  scan E  the model-backed source is exercised, in replay mode:")
    for step in replay_steps:
        print(f"          {step.path}:{step.line_number} replays {step.transcript}")
    print(
        "CHECKED: VER-MOK-018 case L21b, that a workflow step exercises the model-backed "
        "source in\n  replay mode against a committed transcript. The step(s) above are "
        "that case; each named\n  transcript was confirmed to be a file in this "
        "repository, and none of them was read here."
    )
    print(
        "NOT CHECKABLE HERE: whether a provider credential is present in the repository's "
        "Actions\n  secrets. REQ-MOK-073's *Constraints* records that as the condition "
        "this containment rests on,\n  and VER-MOK-018 carries it as an owner attestation."
    )


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--root", default=".", help="repository root")
    arguments = parser.parse_args(argv)

    root = Path(arguments.root).resolve()
    try:
        files, scanned_lines, findings, replay_steps = check(root)
    except Refusal as refusal:
        print(f"REFUSED: {refusal}", file=sys.stderr)
        return 1

    report(root, files, scanned_lines, findings, replay_steps)
    return 1 if findings or replay_step_missing(replay_steps) else 0


if __name__ == "__main__":
    raise SystemExit(main())
