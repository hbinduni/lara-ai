# Dependency security

The dependency-security workflow runs on pull requests, pushes to main, and weekly. Checks fail on reported vulnerabilities; they do not update dependencies or suppress findings.

Rust: `sh scripts/code-vuln.sh` installs cargo-audit 0.22.2 and audits the workspace Cargo.lock.

Dependabot checks supported dependency roots, GitHub Actions and container definitions weekly. Minor and patch updates are grouped; major updates remain separate. Review and validate each update before merging.
