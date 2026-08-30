"""Makes any socket creation in the interpreter that imports this a hard failure.

`VER-MOK-019`'s *Security and privacy* second bullet asks for the no-network claim to be checked
"behaviourally by running a batch with no network available". Removing the machine's network is not
available to this session and would in any case be a weaker reading: a batch that makes no call
passes on an unplugged machine and on a connected one alike, so the unplugged run distinguishes
nothing. What distinguishes is a run in which creating a socket at all raises, which is what this
does. A batch and a classifier that complete normally under it opened no socket.

Placed on `PYTHONPATH`, so it is imported by `site` before the instrument's first line. The engine's
child process is a Rust binary and is unaffected -- rule 19.4's claim about the instruments is the
claim being checked, and `ADR-MOK-001`'s about the library target is not this contract's.
"""

import socket

_LOG = __file__ + ".breaches"


def _refuse(name):
    def guard(*arguments, **keywords):
        with open(_LOG, "a", encoding="utf-8", newline="\n") as handle:
            handle.write(name + "\n")
        raise RuntimeError(f"VER-MOK-019 socket guard: {name} was called")

    return guard


socket.socket = _refuse("socket.socket")
socket.create_connection = _refuse("socket.create_connection")
socket.socketpair = _refuse("socket.socketpair")
socket.getaddrinfo = _refuse("socket.getaddrinfo")
socket.gethostbyname = _refuse("socket.gethostbyname")
