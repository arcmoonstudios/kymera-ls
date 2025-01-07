# Contributing to Kymera

Thank you for your interest in contributing to Kymera! This document provides guidelines and instructions for contributing to the project.

## Development Workflow

1. Fork the repository
2. Create a feature branch from `develop`:
   ```bash
   git checkout develop
   git pull origin develop
   git checkout -b feature/your-feature-name
   ```
3. Make your changes
4. Run tests and ensure CI passes:
   ```bash
   cargo test --all-features --workspace
   cargo fmt --all -- --check
   cargo clippy --all-targets --all-features -- -D warnings
   ```
5. Commit your changes following [Conventional Commits](https://www.conventionalcommits.org/)
6. Push your branch and create a Pull Request against `develop`

## Pull Request Process

1. Update the README.md with details of changes if applicable
2. Add tests for new functionality
3. Ensure all tests pass and there are no clippy warnings
4. Update documentation if needed
5. The PR will be merged once you have the sign-off of at least one maintainer

## Code Style

- Follow Rust standard formatting (enforced by `rustfmt`)
- Follow clippy suggestions
- Write documentation for public APIs
- Include tests for new functionality

## Commit Messages

Follow the Conventional Commits specification:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

Types:
- feat: New feature
- fix: Bug fix
- docs: Documentation only
- style: Code style changes
- refactor: Code changes that neither fix bugs nor add features
- perf: Performance improvements
- test: Adding or modifying tests
- chore: Maintenance tasks

## Getting Help

If you need help, please:
1. Check existing issues
2. Create a new issue with a clear description
3. Tag it appropriately

## License

By contributing, you agree that your contributions will be licensed under the project's MIT License. 