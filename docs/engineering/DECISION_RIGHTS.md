# Decision Rights

AI agents may draft, challenge, decompose, implement, and verify within an approved work order. They do not inherit accountability for product intent, architecture risk acceptance, security exceptions, release authorization, or production operation.

Suggested accountable roles:

- product or domain owner: intent, capability, and requirements;
- technical owner: specification and architecture;
- assurance owner: verification contract and evidence assessment;
- engineering owner: bounded work order;
- release owner: promotion decision;
- service owner: operating assurance.

Automation may prepare `ready` verification and release records from bounded Git observations. Only accountable assurance and release owners may transition those records to `verified` or `released`; record preparation never creates commits, tags, or publications.
