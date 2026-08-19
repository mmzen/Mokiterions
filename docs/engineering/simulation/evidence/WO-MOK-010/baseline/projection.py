"""VER-MOK-010 oracle 1: the projection, in full, as the reviewed artifact.

The projection deletes exactly the text WO-MOK-010 adds to the three record kinds that gained a
field, and nothing else:

  * `,fear:<a>-><b>`  -- the transition form, on `survival_changed` lines;
  * `,fear:<n>`       -- the scalar form, on `agent_initialized` and `action_trace` lines;
  * `,waste_tolerance:<n>` -- the trait, on `agent_initialized` lines.

Each pattern is anchored on the leading comma and on the field name, so it cannot match a position,
an identifier, an event kind, an ordering or another attribute's value. The transition form is
deleted before the scalar form because the scalar pattern is a prefix of it.

It is applied to both captures: to the post-change one so that it can be compared with the
pre-change one, and to the pre-change one so that the projection is proved to be a no-op there. A
projection that changed a pre-change byte would be hiding a difference rather than removing an
addition, which is the one way this oracle could be subverted.
"""

import re

TRANSITION = re.compile(r',fear:\d+->\d+')
TRAIT = re.compile(r',waste_tolerance:\d+')
SCALAR = re.compile(r',fear:\d+')


def project(text):
    """Delete the three added fields from a captured event stream, and nothing else."""
    text = TRANSITION.sub('', text)
    text = TRAIT.sub('', text)
    return SCALAR.sub('', text)
