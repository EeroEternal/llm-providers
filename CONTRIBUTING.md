# Contributing to llm-providers

Thank you for your interest in contributing! This guide will help you get started.

## Guidelines

### Commit Messages

Please use English for all commit messages and follow the [conventional commit](https://www.conventionalcommits.org/) format:

- `feat:` for new features
- `fix:` for bug fixes  
- `chore:` for maintenance tasks
- `docs:` for documentation changes
- `refactor:` for code refactoring

Examples:
```
feat: add Anthropic Claude models
fix: correct OpenAI pricing information
chore: update provider dependencies
```

### Provider Updates

When adding or updating providers:

1. **Edit `data/providers.json`** with your changes
2. **Follow naming conventions**:
   - Use consistent endpoint labels (avoid redundant region suffixes)
   - Region information should be handled by the `region` field only
3. **Ensure data quality**:
   - All required fields must be populated
   - Verify URLs and documentation links are valid
   - Use consistent pricing format

### Testing

Run tests to ensure validity (this will run `build.rs` to embed the registry at compile time):

```bash
cargo test
```

### Code Style

Ensure code follows project formatting:

```bash
cargo fmt --all -- --check
```

## Pull Request Process

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-provider`)
3. Make your changes following the guidelines above
4. Run tests and ensure they pass
5. Commit your changes with proper English commit messages
6. Push to your fork and submit a Pull Request
7. Provide a clear description of changes in the PR

## Review Process

All PRs will be reviewed for:
- ✅ Correct data format and structure
- ✅ Proper testing and validation
- ✅ Code style compliance
- ✅ Clear commit messages in English
- ✅ Documentation updates if needed

## Questions?

If you have questions, feel free to open an issue for discussion before starting your contribution.
