# Deterministic replay

Two identical runs per declared verification seed — `VER-MOK-005`'s set of 0, 1, 42, 123 and 777 — at 400
ticks each, through the engine binary, with both the structured record stream written by `--events-path`
and the standard output compared as bytes.

**All five seeds reproduced identically in both streams.** No difference in any of the twenty digests.

## What this establishes and what it does not

It does **not** establish that the observer changes nothing. That is `VER-MOK-005`'s non-perturbation
property, measured by comparing an observed run against an unobserved one, and it is undisturbed by this
work order: no engine call site changed, and the footer reads the configuration and the retained buffer by
copy into a value struct that owns its fields.

What it does establish is the precondition. If the engine were not reproducible on this tree, no
non-perturbation comparison against it would mean anything, and the presentation change this work order
carries could not be shown to be confined to presentation. `git diff --stat -- mokiterions-core` being
empty is the other half of that argument.

## Full run

```
Two identical runs per declared verification seed, 400 ticks each, compared byte for byte.

The declared verification seed set is VER-MOK-005's: 0, 1, 42, 123 and 777. Each seed is run
twice through the engine binary with the same options, and both the structured record stream
written by --events-path and the standard output are compared as bytes. A difference in either
would mean the tree is not reproducible, and no non-perturbation comparison on it would mean
anything.

seed 0
  record stream    1172382 bytes  sha256 35cd2c6a172e634f635d631739694463b1d26882da5f58d64c8c8bae75dcd798
                   1172382 bytes  sha256 35cd2c6a172e634f635d631739694463b1d26882da5f58d64c8c8bae75dcd798
                  identical: yes
  standard output   552526 bytes  sha256 ac0615fd177fa191a10aaf6ff92c47e901f3a1b891152ddcb8a63b89d56badd6
                    552526 bytes  sha256 ac0615fd177fa191a10aaf6ff92c47e901f3a1b891152ddcb8a63b89d56badd6
                  identical: yes

seed 1
  record stream    1191037 bytes  sha256 577d3ef52dd4f24a2e822c64f1bbd9f0d8f4ee115aefea19e4f6fdaa8a4379a3
                   1191037 bytes  sha256 577d3ef52dd4f24a2e822c64f1bbd9f0d8f4ee115aefea19e4f6fdaa8a4379a3
                  identical: yes
  standard output   561269 bytes  sha256 8672fe42d5871039eed4bfb201f9b140fa07286e697b1a252c6758edb25810b1
                    561269 bytes  sha256 8672fe42d5871039eed4bfb201f9b140fa07286e697b1a252c6758edb25810b1
                  identical: yes

seed 42
  record stream    1190042 bytes  sha256 75176569ce26b3d2fbb5a80262d436acfe8a11426ae8789e63c177622b69a42a
                   1190042 bytes  sha256 75176569ce26b3d2fbb5a80262d436acfe8a11426ae8789e63c177622b69a42a
                  identical: yes
  standard output   561063 bytes  sha256 dde7462440eda3a0c35e8bae91f34237354623422d945d8555501a1566cc8a1a
                    561063 bytes  sha256 dde7462440eda3a0c35e8bae91f34237354623422d945d8555501a1566cc8a1a
                  identical: yes

seed 123
  record stream    1123108 bytes  sha256 983dbba4d2f7d9db4c5fcfc79e9d91689ccef69ef6ebae505a871e43d0eddaf8
                   1123108 bytes  sha256 983dbba4d2f7d9db4c5fcfc79e9d91689ccef69ef6ebae505a871e43d0eddaf8
                  identical: yes
  standard output   524925 bytes  sha256 00d435090d64b7cd38663a5c5dfbcd7d5b4ba9ad49518a618df09cb4e899023b
                    524925 bytes  sha256 00d435090d64b7cd38663a5c5dfbcd7d5b4ba9ad49518a618df09cb4e899023b
                  identical: yes

seed 777
  record stream    1191742 bytes  sha256 eb1c9382d982b0eb391db48e3d97c455e545a1091c41c35f92a52be736e3c077
                   1191742 bytes  sha256 eb1c9382d982b0eb391db48e3d97c455e545a1091c41c35f92a52be736e3c077
                  identical: yes
  standard output   562510 bytes  sha256 1811b0245778bf1e5056be15c59e60fb8e2bae0de7d7cef10846b08c1cc87a9a
                    562510 bytes  sha256 1811b0245778bf1e5056be15c59e60fb8e2bae0de7d7cef10846b08c1cc87a9a
                  identical: yes

5 of 5 seeds reproduced identically in both streams.

What this does not establish: that the observer changes nothing. That is VER-MOK-005's
non-perturbation property, measured by comparing an observed run against an unobserved one, and
it is unaffected by this work order — no engine call site changed, and the footer reads the
configuration and the retained buffer by copy. What this does establish is the precondition:
the engine is reproducible on this tree, so any such comparison is meaningful.
```
