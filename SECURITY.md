# Security Policy

## Supported Versions

`executainer` is pre-1.0.

Security fixes are applied to the latest release on `main`. Older tags may not receive backports.

## Reporting A Vulnerability

Please do not open a public issue for security-sensitive reports.

Email: `security@dgit.co`

Include:

- affected version or commit
- reproduction steps
- impact
- any suggested mitigation

We will acknowledge receipt as quickly as possible and coordinate a fix before public disclosure when appropriate.

## Security Boundaries

`executainer` is an orchestration layer, not a sandbox.

Important non-goals for v1:

- full process isolation
- policy-based approval evaluation
- remote multi-tenant execution

If you use writable lanes, assume the underlying agent tool and local machine security model still matter.
