# Security Policy for awald-core

## Supported Versions

| Version | Supported          | Security Updates |
|---------|-------------------|------------------|
| 0.1.x   | :white_check_mark: | :white_check_mark: |
| < 0.1.0 | :x:                | :x:              |

## Reporting a Vulnerability

**Please do not report security vulnerabilities through public GitHub issues.**

If you discover a security vulnerability, please report it privately:

### Primary Method
- **Email**: security@awald.app
- **Expected Response Time**: Within 48 hours

### Alternative Method
- **GitHub Security Advisory**: Use the "Report a vulnerability" feature on this repository

### What to Include
- Description of the vulnerability
- Steps to reproduce
- Potential impact assessment
- Any proposed mitigation (if known)
- Affected versions (if known)

## Response Process

1. **Acknowledgment**: We'll acknowledge receipt within 48 hours
2. **Investigation**: We'll investigate and assess the vulnerability
3. **Remediation**: We'll develop and test a fix
4. **Disclosure**: We'll coordinate disclosure with you
5. **Patch Release**: We'll release a security patch
6. **Public Advisory**: We'll publish a security advisory (if appropriate)

## Security Best Practices

### For Users
- Keep dependencies updated (Dependabot will help)
- Review security advisories
- Use the latest stable version
- Monitor GitHub Security Advisories for this repository

### For Developers
- Follow secure coding practices
- Use dependency scanning tools
- Report vulnerabilities responsibly
- Review Python code execution security implications
- Validate all user inputs before Python execution

## Security Scanning

This project uses automated security scanning:
- **Dependabot**: Continuous dependency monitoring for Rust crates
- **Security Audit**: Daily vulnerability scanning with `cargo audit`
- **CodeQL**: Static analysis for security issues
- **Python Security**: Regular review of pyo3 usage and script execution

### Specific Security Considerations

#### Python Execution Security
- Scripts are executed in isolated Python environments
- No filesystem access by default (can be configured)
- No network access by default (can be configured)
- Scripts with null bytes are rejected
- Execution is serialized to prevent race conditions

#### Dependencies
- All Rust dependencies are scanned for known vulnerabilities
- Python dependencies (pyo3, polars) are regularly updated
- Workspace-level dependency management prevents version conflicts

## Security Updates

Security updates are handled through:
- Patch releases for supported versions
- Security advisories with CVE numbers (when applicable)
- Upgrade guides for affected versions
- Dependabot pull requests for dependency updates

## Severity Classification

- **Critical**: Remote code execution, data exfiltration
- **High**: Privilege escalation, data modification
- **Medium**: Information disclosure, denial of service
- **Low**: Informational issues, best practice violations

## Contact

For security-related questions:
- **Security Team**: security@awald.app
- **General Issues**: Use GitHub issues (non-security matters only)

## See Also

- [CONTRIBUTING.md](docs/CONTRIBUTING.md) - Contribution guide
- [PRE_COMMIT.md](docs/PRE_COMMIT.md) - Code quality setup
- [TODO.md](docs/TODO.md) - Development roadmap
- [README.md](README.md) - Project overview

## Related Documentation

- [SECURITY.md](./SECURITY.md) - This security policy
- [docs/TODO.md](./docs/TODO.md) - Development roadmap with security considerations
- [README.md](./README.md) - General project information
