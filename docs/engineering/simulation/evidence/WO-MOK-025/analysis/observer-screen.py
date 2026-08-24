"""Reconstruct one observer screen from the drawing stream the observer wrote.

Usage, from anywhere:

    python docs/engineering/simulation/evidence/WO-MOK-025/analysis/observer-screen.py \
        <captured-stdout> <output-file>

## Why this exists

`VER-MOK-018` case **L31** and `WO-MOK-025`'s completion report item 5 both ask what the observer's
panes did while replaying a transcript. The observer draws into a terminal, so the obvious capture -
run it with standard output redirected to a file - produces a file that is mostly escape sequences
and in which no pane's text is contiguous: ratatui's diffing backend emits a cursor-move before
almost every cell, so the footer's `source llm` is stored as `s`, a cursor-move, `o`, a cursor-move,
and so on. Searching that file for `source llm` finds nothing, and concluding from that that the
footer did not name the source would be wrong.

This script replays the stream into a character grid and writes the grid out, which is the frame the
operator would have seen. It is not a terminal emulator: it honours the cursor-position escape,
carriage return and newline, and it discards every other escape sequence, because those are colour,
attribute and screen-mode changes that do not move the cursor and so cannot move a character. If
`ratatui` ever emitted relative cursor movement instead, this would silently misplace text; that is
why the reconstruction is checked against a second, independent reading of the same run rather than
trusted on its own - `candidate/replay-identity.txt` compares eight figures on the reconstructed
screen against the engine binary's own summary line for the same transcript.

## The capture this reads

The observer has no non-interactive exit: nothing ends the program but a key press, and on Windows
`crossterm` reads the console input buffer rather than the standard input handle, so keys cannot be
piped in. The capture is therefore taken by letting the observer reach the transcript's horizon and
then killing it, and the reconstruction is of the last frame drawn before the kill. The process's
exit status is the killer's and says nothing about the run; what says the run finished is the frame
itself, which states `finished tick_limit`.
"""

import pathlib
import re
import sys

# Wider and taller than any viewport this is used at, so a stray coordinate lands inside the grid
# and is visible in the output rather than raising.
WIDTH = 200
HEIGHT = 60

# A CSI sequence: `ESC [`, its parameter bytes, its final byte. `H` is cursor position, which is the
# one this cares about; every other final byte is consumed and ignored.
CSI = re.compile(r"\[([0-9;?]*)([a-zA-Z])")


def reconstruct(raw: str) -> list[str]:
    """The character grid the stream drew, as rows with trailing blanks removed."""
    grid = [[" "] * WIDTH for _ in range(HEIGHT)]
    row = column = 0
    index = 0
    while index < len(raw):
        character = raw[index]
        if character == "\x1b":
            match = CSI.match(raw, index + 1)
            if match is None:
                # A two-character escape, or an escape this does not model. Neither moves the
                # cursor, so skipping both bytes is correct.
                index += 2
                continue
            if match.group(2) == "H":
                # `ESC [ row ; column H`, one-based, with either parameter defaulting to 1.
                parameters = (
                    [int(value) if value else 1 for value in match.group(1).split(";")]
                    if match.group(1)
                    else [1, 1]
                )
                row = parameters[0] - 1
                column = parameters[1] - 1 if len(parameters) > 1 else 0
            index = match.end()
            continue
        if character == "\n":
            row += 1
            column = 0
            index += 1
            continue
        if character == "\r":
            column = 0
            index += 1
            continue
        if 0 <= row < HEIGHT and 0 <= column < WIDTH:
            grid[row][column] = character
        column += 1
        index += 1

    rows = ["".join(cells).rstrip() for cells in grid]
    while rows and not rows[-1]:
        rows.pop()
    return rows


def main(argv: list[str]) -> int:
    if len(argv) != 3:
        print(__doc__)
        return 2
    source, destination = pathlib.Path(argv[1]), pathlib.Path(argv[2])
    # `replace` rather than `strict`: a capture killed mid-frame can end inside a multi-byte glyph,
    # and a reconstruction that raises on the last byte of the file is less useful than one that
    # shows the frame with one glyph marked.
    raw = source.read_bytes().decode("utf-8", "replace")
    rows = reconstruct(raw)
    # Written LF, because this packet's evidence tree is `-text` in `.gitattributes` and is stored
    # exactly as written.
    destination.write_text("\n".join(rows) + "\n", encoding="utf-8", newline="\n")
    print(f"{source}: {len(raw)} character(s) -> {destination}: {len(rows)} row(s)")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
