# A6 - no version reference is left behind

Search for the superseded version as a version to install, over tracked files,
excluding this evidence directory.

```text
./docs/engineering/simulation/evidence/WO-MOK-014/merge/harness.sh:102:  echo "Doctor, under the pinned 0.4.0 venv"
./docs/engineering/simulation/evidence/WO-MOK-014/merge/harness.sh:105:  echo "    \$ <pinned 0.4.0 venv>/python -m se_harness doctor <tree>"
./docs/engineering/simulation/evidence/WO-MOK-014/merge/harness.sh:111:  echo "The pinned 0.4.0 reading is the one .github/workflows/engineering-harness.yml installs by exact"
./docs/engineering/simulation/evidence/WO-MOK-014/merge/harness.sh:113:  echo "repository whose declared runtime is 0.4.0, at this commit as at every other; ../WO-MOK-014-"
./docs/engineering/simulation/evidence/WO-MOK-014/merge/harness.sh:117:  echo "Preflight, review phase, under the pinned 0.4.0 venv"
./docs/engineering/simulation/evidence/WO-MOK-014/merge/harness.sh:120:  echo "    \$ <pinned 0.4.0 venv>/python -m se_harness preflight <tree> --work-order WO-MOK-014 --phase review"
./scripts/test_check_release_authorization.py:47:tool_version = "0.4.0"
```

The repository-owned pin now reads:

```text
64:  SE_HARNESS_VERSION: "0.8.0"
3:tool_version = "0.8.0"
```
