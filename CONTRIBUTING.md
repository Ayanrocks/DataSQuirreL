# Contributing to DataSQuirreL

First off, thank you for considering contributing to DataSQuirreL! It's people like you that make DataSQuirreL such a great tool for the community.

## Code of Conduct

By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md). We expect all contributors to maintain a welcoming and inclusive environment.

## How Can I Contribute?

### Reporting Bugs
If you find a bug, please open an issue in the GitHub repository. Provide as much detail as possible, including:
- Steps to reproduce the bug.
- The expected behavior vs. the actual behavior.
- System specifications (OS, App version, Node/Rust versions).
- Logs, screenshots, or error traces if applicable.

### Suggesting Enhancements
Have an idea to make DataSQuirreL better? We'd love to hear it! Open an issue formatted as a feature request, explaining the motivation, use cases, and how the feature should behave.

### Pull Requests
We gladly accept Merge/Pull Requests! Here is the workflow:

1. **Fork the repository** to your own GitHub account and clone it to your local machine.
2. **Create a branch** for your feature or bug fix: `git checkout -b feat/my-awesome-feature` or `git checkout -b fix/issue-number`.
3. **Write tests** where applicable to cover your changes.
4. **Make sure all tests pass** locally before submitting your PR.
5. **Follow Conventional Commits** for your commit messages (e.g., `feat: add awesome feature`, `fix: resolve crash on startup`).
6. **Push to your fork** and open a Pull Request against the `main` branch.

In your PR description, explain what the PR does, link to the relevant issue, and add screenshots if your PR includes UI changes.

## Development Setup

To set up the development environment locally:

1. Install [Node.js](https://nodejs.org/) (v18+ recommended) and `npm`.
2. Install the Rust toolchain (via [rustup](https://rustup.rs/)).
3. Follow the Tauri prerequisites for your specific OS (available on the [Tauri getting started page](https://tauri.app/v1/guides/getting-started/prerequisites)).
4. Clone your fork and run the following in the project root:

```bash
npm install
npm run tauri dev
```

### Formatting and Linting
Please format and lint your code before committing. The project uses standard ESLint and Prettier for the frontend (Svelte/TS) and standard Cargo fmt for the Rust backend.
- Format frontend code: `npm run format`
- Lint frontend code: `npm run lint`

Thank you for your contributions!
