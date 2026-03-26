# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x   | Yes       |

## Reporting a Vulnerability

If you discover a security vulnerability in Jantu, please report it responsibly:

1. **Do not** open a public issue
2. Email the maintainers with a description of the vulnerability
3. Include steps to reproduce if possible
4. Allow reasonable time for a fix before public disclosure

## Security Practices

- `cargo audit` is run as part of the standard check suite
- `cargo deny` enforces license and source restrictions
- Zero `unwrap`/`panic` policy in library code prevents unexpected crashes
- All inputs are clamped to valid ranges at API boundaries
- No `unsafe` code is used in this crate
