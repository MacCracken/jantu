# Security Policy

## Supported Versions

| Version | Supported          |
|---------|--------------------|
| 1.0.x   | Yes                |
| < 1.0   | No                 |

## Reporting a Vulnerability

If you discover a security vulnerability in Jantu, please report it responsibly:

1. **Do not** open a public issue
2. Use [GitHub Security Advisories](https://github.com/MacCracken/jantu/security/advisories) to report privately
3. Include steps to reproduce if possible

### Response SLA

| Severity | Acknowledgement | Fix Target  |
|----------|----------------|-------------|
| Critical | 48 hours       | 14 days     |
| High     | 48 hours       | 30 days     |
| Moderate | 5 days         | Next release |
| Low      | 5 days         | Next release |

We follow coordinated disclosure. Reporters are credited unless they prefer anonymity.

## Scope

This policy covers the `jantu` crate and its published API. Optional features (`logging`, `personality`) extend the scope to their respective integration points.

## Security Practices

- **Zero `unsafe` code** — no unsafe blocks anywhere in the crate
- **Zero `unwrap`/`panic`** — all errors return `Result`, all inputs clamped to valid ranges
- **`cargo audit`** run as part of the standard check suite
- **`cargo deny`** enforces license and source restrictions (crates.io only)
- **`no_std` by default** — minimal attack surface; no I/O, no networking, no filesystem
- **Minimal dependencies** — 4 required crates (serde, thiserror, tracing, alloc)

## Per-Module Risk Assessment

| Module | Risk | Mitigation |
|--------|------|------------|
| All math functions | f32 precision limits | All outputs clamped to documented ranges |
| `instinct` | Priority overflow from extreme multipliers | `priority` clamped to [0.0, 1.0] |
| `survival` | `partial_cmp` returning `None` for NaN | Fallback to `Ordering::Equal` |
| `lifecycle` | Zero/negative mass | Early return with 0.0 |
| `circadian` | Zero period division | Early return with 1.0 |
| `genetics` | Mutation accumulation drift | All trait values clamped to [0.0, 1.0] |
| `coevolution` | Unbounded encounter rates | Inputs clamped, density must be positive |
| `stress` | Resilience depletion spiral | Resilience clamped to [0.0, 1.0], recovery possible |
