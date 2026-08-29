# Item 12: the protocol document as shipped, and the check that it is sufficient

Measured at candidate `6e9ca13` on 2026-08-29.

## The document as shipped

`docs/CONNECTOR_PROTOCOL.md`, **18,140 bytes, 295 lines**, addressed to a connector's author and not to a
reader of this repository's specifications. Its sections:

```
# The Mokiterions connector protocol
## What a connector is for
## How your connector is started
## Framing
## The request object
## The response object
## The action grammar
## The credential
## Two limits, stated rather than defended
## A worked exchange
## Checklist for a new connector
## The canned connector
```

## The check: the document fixes every field the host uses

Item 12 asks for evidence that the connector was written against the document rather than against the
engine's internals, "the check being that the document alone is sufficient". Sufficiency is checked here as
**closure of the field set**: every field the host writes on a request and every field it reads from a
response is fixed by the document, so nothing an author needs is reachable only from the source.

The host's request is one `format!` in `ConnectorPort::request_line`, and it writes **five** fields:

| field written | fixed by the document |
|---|---|
| `protocol` | *The request object* table, "Protocol version. `1` today." |
| `tick` | same table, "The simulation tick this opportunity belongs to. Informational." |
| `actor` | same table, "The Mokiterion's identifier, e.g. `"M03"`." |
| `prompt` | same table, "**The entire prompt.** Send it to the model unchanged." |
| `schema` | same table, "A JSON Schema describing the response `action` object." |

And the document says so in the same words the source does: "Those five are all of them." The type behind it
has no field to put a sixth in.

The host's reading of a response is `ConnectorPort::interpret`, `action`, `usage` and `retain_binding`, and
the complete set of members it consults is **ten**:

| member read | fixed by the document |
|---|---|
| `protocol` | *The response object* table |
| `action.verb` | same table, "One of the eleven verbs below" — and the eleven are enumerated in *The action grammar* |
| `action.parameter` | same table, "Present only for a verb that takes one" |
| `model` | same table, "**The model that answered**, as the provider identified it" |
| `reasoning` (the level, a string) | same table, and the document warns that `reasoning` "appears at two levels and means two different things" |
| `usage.prompt` | same table |
| `usage.cached_prompt` | same table, with the instruction to report `0` rather than invent a figure |
| `usage.output` | same table |
| `usage.reasoning` (the count, an integer) | same table |
| `error.kind` and `error.message` | *Error*: "`kind` is one of `transport`, `provider`, `malformed` or `refused`" |

**The set is closed.** There is no member the host consults that the document does not fix, and no field the
document fixes that the host ignores. That is the strongest form this check can take from inside the
repository: an author who implements the document's two tables has implemented everything the host will send
and everything it will read, and an author who implements them and nothing else cannot be surprised.

Beyond the field set the document also fixes the three things a field table cannot: the framing (one object
per line, no newline within an object, one response per request, in order), the start-up contract (executable
by the platform, no arguments, exit `0` at end of input), and the credential rule (read from the connector's
own environment, emitted on no stream, in no message, ever).

## The canned connector names no engine internal

`mokiterions-core/tests/support/canned_connector.rs` imports exactly `std::env`, `std::io` and
`std::process::ExitCode`. It contains **no `use mokiterions::`** and **no `crate::` path**; its three
case-insensitive occurrences of `mokiterions` are two lines of module prose and the name of the environment
variable it reads. It could not have been written against the engine's internals, because it does not name
one.

That is a fact about the artifact rather than about how it was produced, and it is the fact worth having: a
fixture that reached into the engine's types would compile and pass, and its passing would establish nothing
about a program on the operator's machine that cannot.

## The honest limit on this evidence

**Neither connector in this initiative was written by an author who had only the document.** The canned
connector and the connector that made the live run were both written by the implementation agent, with the
engine's source in view. So this file does not claim that the document was *used* as the only source; it
claims the two things it can measure — that the document fixes every field the host uses, and that neither
connector's source names an engine item.

What would settle the remaining question is a connector written by someone with no access to this repository.
`WO-MOK-026`'s stop-and-escalate condition 1 names the failure it is guarding against — "the line protocol
cannot be documented well enough for a third party to implement a connector from the document alone" — and
that condition was not reached: the document is complete against the host's behaviour, measured above. But
"not reached" is not the same as "demonstrated by a third party", and no evidence in this packet is that
demonstration.

The document's *Checklist for a new connector* is twelve items an author can work through, and *A worked
exchange* shows one request and one response in full. Both exist so that the missing demonstration is as
cheap as possible for whoever eventually makes it.
