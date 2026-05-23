# owner-signal-domain-criome Architecture

`owner-signal-domain-criome` is the owner-only Signal contract for the
`domain-criome` component. It controls domain registration, delegation,
retirement, and projection policy.

## Boundary

The ordinary `signal-domain-criome` contract resolves and projects domain
meaning. This owner contract mutates the registry that gives that meaning
authority.

Provider-specific plan application remains outside this contract. The domain
registry can decide what should exist; `cloud` decides how a provider applies
it.

## Public Operations

- `RegisterDomain(Registration)` registers a domain root.
- `Delegate(Delegation)` delegates a named branch.
- `RegisterAuthority(AuthorityRegistration)` records an off-daemon authority
  endpoint for a domain whose own daemon should answer resolution.
- `RetireDomain(Retirement)` retires a registered domain.
- `SetPolicy(Policy)` changes projection policy.

## Owns

- Domain-registration authority.
- Delegation authority.
- Off-daemon authority endpoint registration.
- Projection-policy directives.
- Typed owner rejections.

## Does Not Own

- Cloudflare, Google, Hetzner, or other provider vocabulary.
- Provider credentials.
- External API mutation.
- The runtime daemon's actor tree or database.

## Constraints

- Depend on `signal-frame`, not deprecated `signal-core`.
- Reuse public domain types from `signal-domain-criome`.
- Keep provider names out of this contract.
