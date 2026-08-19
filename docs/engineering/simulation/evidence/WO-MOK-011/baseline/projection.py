"""VER-MOK-011 oracle 1: the projection, in full, as the reviewed artifact.

The projection deletes exactly the text `WO-MOK-011` adds to the one record kind that gained a
field, and nothing else:

  * `result=name:<letters>,` -- the name, the first detail of an `agent_initialized` line.

The pattern is anchored on three sides. It requires the literal `result=` immediately before it, so
it cannot match anywhere but at the head of a record's detail list; it requires the field name
`name:`; and it requires the trailing comma that separates the name from the detail that follows.
The value is restricted to one to five ASCII letters, which is the domain `REQ-MOK-040` fixes. A
position, an identifier, an event kind, a territory, an attribute value, an ordering or a whole line
cannot match it: every one of those either contains a character outside `A-Za-z` or does not sit
between `result=` and a comma.

It is applied to both captures: to the post-change one so that it can be compared with the
pre-change one, and to the pre-change one so that the projection is proved to be a no-op there. A
projection that changed a pre-change byte would be hiding a difference rather than removing an
addition, which is the one way this oracle could be subverted.

The projection is deliberately not anchored on the event kind. `REQ-MOK-040` requires the name to be
reported on `agent_initialized` lines and nowhere else, and that claim is verified by its own case in
the matrix, by counting occurrences. Anchoring here as well would let a name leak onto a second
record kind and be silently deleted by the projection that was supposed to expose it.
"""

import re

NAME = re.compile(r'result=name:[A-Za-z]{1,5},')


def project(text):
    """Delete the added name field from a captured event stream, and nothing else."""
    return NAME.sub('result=', text)
