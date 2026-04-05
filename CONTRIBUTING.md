# Contributing to AI Agent Orchestrator

Thank you for your interest in contributing! This document provides guidelines and information for contributors.

## 🎯 Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/ai-agent-orchestrator.git
   cd ai-agent-orchestrator
   ```
3. **Create a branch** for your changes:
   ```bash
   git checkout -b feature/your-feature-name
   ```

4. **Set up the development environment**:
   - Backend: Install Rust 1.75+ and run `cargo check`
   - Frontend: Run `npm install` in the `frontend/` directory

## 🏗️ Development Workflow

### Backend Development

```bash
cd backend

# Run tests
cargo test

# Run with environment
OPENROUTER_API_KEY=your_key cargo run

# Format code
cargo fmt

# Lint
cargo clippy
```

### Frontend Development

```bash
cd frontend

# Install dependencies
npm install

# Run development server
npm run dev

# Run tests
npm test

# Build
npm run build

# Type check
npx tsc --noEmit
```

## 📝 Code Style

### Rust
- Use `cargo fmt` before committing
- Follow the style guide in `rustfmt.toml`
- Write documentation for public APIs
- Prefer `Result` over panics; use `anyhow::Result` for application errors

### TypeScript/React
- Use 2-space indentation
- Prefer functional components with hooks
- Write TypeScript interfaces for all data structures
- Use named exports
- Follow ESLint configuration

### Git Commits
- Use conventional commits: `feat:`, `fix:`, `docs:`, `test:`, `refactor:`, `chore:`
- Write clear, concise commit messages
- Reference issues in the format `#123`

## 🧪 Testing

### Backend
- Unit tests: `cargo test`
- Integration tests: `cargo test --test integration`
- Ensure all tests pass before submitting PR

### Frontend
- Unit tests with Vitest
- Run `npm test` before committing
- Add tests for new components and utilities

## 🔍 Pull Request Process

1. **Update your fork** to the latest upstream changes
2. **Run all tests** and ensure they pass
3. **Update documentation** if needed (README, API docs)
4. **Create a Pull Request** with:
   - Clear description of changes
   - Link to related issues
   - Screenshots for UI changes
   - Test evidence (manual or automated)

5. **Code Review**:
   - Address review comments
   - Keep PRs focused on a single concern
   - Squash commits before merging if needed

## 🐛 Bug Reports

When reporting bugs, include:
- Steps to reproduce
- Expected vs actual behavior
- Screenshots/logs if applicable
- Environment details (OS, browser, versions)

## 💡 Feature Requests

Feature requests are welcome! Please:
- Check existing issues to avoid duplicates
- Provide a clear use case
- Consider implementation complexity

## 📚 API Design Guidelines

When designing new API endpoints:
- Use RESTful conventions
- Return appropriate HTTP status codes
- Include pagination for list endpoints
- Document error responses
- Version your endpoints (`/api/v1/`, `/api/v2/`)
- Consider idempotency for write operations

## 🔐 Security

- Never commit secrets or credentials
- Use environment variables for configuration
- Follow least-privilege principle
- Report security vulnerabilities privately

## 📜 Code of Conduct

This project follows the [Contributor Covenant](https://www.contributor-covenant.org/). By participating, you agree to abide by its terms.

## ❓ Questions

- Check existing documentation first
- Search open/closed issues
- Open a new issue with `question` label if needed

## 🙏 Acknowledgments

Thank you to all contributors!

---

**Need help?** Open an issue or reach out to the maintainers.
