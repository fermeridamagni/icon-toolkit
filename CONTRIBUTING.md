# 🤝 Contributing to `icon-toolkit`

Thank you for your interest in contributing to **Icon Toolkit**! We welcome contributions from the community to help make multi-platform icon generation, format conversion, and AI icon synthesis faster, simpler, and more capable.

---

## 📑 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Development Environment Setup](#development-environment-setup)
  - [Prerequisites](#prerequisites)
  - [Initial Setup](#initial-setup)
- [Repository Architecture](#repository-architecture)
- [Development Workflows & Build Scripts](#development-workflows--build-scripts)
  - [1. Building Rust Crates & CLI](#1-building-rust-crates--cli)
  - [2. Building TypeScript / NAPI Native Bindings](#2-building-typescript--napi-native-bindings)
  - [3. Running Tests](#3-running-tests)
  - [4. Linting & Code Formatting](#4-linting--code-formatting)
- [Coding Standards & Conventions](#coding-standards--conventions)
  - [Rust Code Standards](#rust-code-standards)
  - [TypeScript Code Standards (Ultracite)](#typescript-code-standards-ultracite)
  - [Documentation & Explanation](#documentation--explanation)
- [How to Add New Features](#how-to-add-new-features)
  - [Adding a New Target Platform](#adding-a-new-target-platform)
  - [Adding a New Image Format](#adding-a-new-image-format)
  - [Adding a New AI Provider](#adding-a-new-ai-provider)
- [Commit Message Guidelines](#commit-message-guidelines)
- [Submitting a Pull Request](#submitting-a-pull-request)

---

## 📜 Code of Conduct

We are committed to providing a welcoming, inclusive, and harassment-free environment for everyone. Please be respectful and constructive in all interactions, issues, and pull request discussions.

---

## 🛠 Development Environment Setup

### Prerequisites

Make sure you have the following installed on your machine:

1. **Rust & Cargo** (Edition 2021, v1.75+):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   rustup component add clippy rustfmt
   ```
2. **Bun** (v1.3+ — our standard package manager and runtime):
   ```bash
   curl -fsSL https://bun.sh/install | bash
   ```
3. **UV** (Optional, required if working on Python helper scripts):
   ```bash
   curl -LsSf https://astral.sh/uv/install.sh | sh
   ```

### Initial Setup

1. Fork and clone the repository:
   ```bash
   git clone https://github.com/<your-username>/icon-toolkit.git
   cd icon-toolkit
   ```

2. Install Bun dependencies:
   ```bash
   bun install
   ```

3. Build all workspace crates and verify tests:
   ```bash
   cargo test --workspace
   ```

---

## 🏛 Repository Architecture

`icon-toolkit` follows a modular architecture with clean separation of concerns:

```txt
.
├── cli/                          # Rust CLI executable (`icon-toolkit-cli`)
│   ├── Cargo.toml
│   └── src/main.rs               # Clap CLI argument parsing & command dispatch
│
├── crates/
│   └── icon-toolkit-core/        # Pure Rust library engine
│       ├── Cargo.toml
│       ├── src/
│       │   ├── lib.rs            # Public API exports
│       │   ├── types.rs          # Data structures, enums (IconTarget, Mode, AiProvider)
│       │   ├── error.rs          # Custom error types (`IconToolkitError`)
│       │   ├── generator.rs      # Multi-target generation logic & manifest builders
│       │   ├── converter.rs      # Format conversion (PNG, WebP, SVG, ICO, JPG)
│       │   └── ai.rs             # BYOK AI provider clients (OpenAI, Stability, Gemini)
│       └── tests/
│           └── integration_tests.rs # Rust integration tests with `demo-icon.png`
│
├── package/                      # TypeScript package with NAPI-RS native bindings
│   ├── Cargo.toml                # NAPI-RS `cdylib` crate (`icon-toolkit-napi`)
│   ├── package.json              # Package metadata and NAPI build scripts
│   ├── src/lib.rs                # NAPI-RS JS/Rust boundary functions
│   ├── index.ts                  # TypeScript API entry point and native binary loader
│   └── cli.ts                    # Bun-executable CLI wrapper
│
├── skills/                       # Built-in Agent Skill for Antigravity & AI assistants
│   └── icon-toolkit/
│       ├── SKILL.md              # Skill documentation and invocation guide
│       └── evals/evals.json      # Evals configuration
│
├── tests/
│   └── ts-package.test.ts        # TypeScript API integration tests (Bun test runner)
│
├── biome.jsonc                   # Biome / Ultracite linter and formatter config
├── Cargo.toml                    # Root Cargo workspace manifest
└── ULTRACITE.md                  # Ultracite code standard guidelines
```

---

## ⚙️ Development Workflows & Build Scripts

### 1. Building Rust Crates & CLI

```bash
# Check compilation across all workspace crates
cargo check --workspace

# Build debug binary
cargo build --workspace

# Build optimized release binary
cargo build --release --workspace

# Run CLI directly during development
cargo run --bin icon-toolkit -- generate -i demo-icon.png -o output/test-icons
```

### 2. Building TypeScript / NAPI Native Bindings

Whenever you modify `crates/icon-toolkit-core` or `package/src/lib.rs`, recompile the NAPI native binding:

```bash
# Build release NAPI binary (.node extension)
cd package
bun run build
cd ..

# Or for debug builds:
cd package
bun run build:debug
cd ..
```

### 3. Running Tests

Both the Rust engine and TypeScript wrapper have dedicated test suites:

```bash
# Run Rust unit and integration tests
cargo test --workspace

# Run specific integration test
cargo test -p icon-toolkit-core --test integration_tests

# Run TypeScript tests with Bun
bun test
```

### 4. Linting & Code Formatting

All code must pass strict linting and formatting standards before being merged:

```bash
# Check Rust formatting
cargo fmt --check

# Format Rust code
cargo fmt

# Check Rust Clippy lints (warnings treated as errors)
cargo clippy --workspace -- -D warnings

# Check TypeScript code with Ultracite (Biome)
bun run check

# Automatically fix TypeScript lint & format issues
bun run fix
```

---

## 📐 Coding Standards & Conventions

### Rust Code Standards

- **Edition**: Use standard Rust 2021 edition idioms.
- **Error Handling**: Use `thiserror` for library error definitions (`crates/icon-toolkit-core/src/error.rs`) and `anyhow` for CLI error context (`cli/src/main.rs`). Avoid `.unwrap()` or `.expect()` in library code.
- **Clippy**: Code must compile with `cargo clippy --workspace -- -D warnings` with zero warnings.
- **Documentation**: Document all public structs, enums, functions, and modules with doc comments (`///` and `//!`).

### TypeScript Code Standards (Ultracite)

- **Always use TypeScript** instead of JavaScript.
- **Always use Bun** as the runtime and package manager (do not use npm/yarn/pnpm/node).
- **Strict Typing**: Avoid `any` types; prefer strict interfaces and type narrowing.
- **Ultracite Compliance**: Follow [ULTRACITE.md](ULTRACITE.md) code quality standards. Run `bun run fix` to resolve common issues automatically.

### Documentation & Explanation

- Every non-trivial function or module should clearly explain **why** the code is written and what purpose it serves.
- Keep comments up to date when modifying existing logic.

---

## 🚀 How to Add New Features

### Adding a New Target Platform

1. **Update Types**: Add the new variant to `IconTarget` enum in [`crates/icon-toolkit-core/src/types.rs`](crates/icon-toolkit-core/src/types.rs).
2. **Implement Generation Logic**: Add a target handler function in [`crates/icon-toolkit-core/src/generator.rs`](crates/icon-toolkit-core/src/generator.rs) (specifying sizes, directories, manifests, and file naming).
3. **Expose in NAPI**: Ensure `package/src/lib.rs` and `package/index.ts` forward the new target string.
4. **Add Tests**: Add test assertions in [`crates/icon-toolkit-core/tests/integration_tests.rs`](crates/icon-toolkit-core/tests/integration_tests.rs) and [`tests/ts-package.test.ts`](tests/ts-package.test.ts).
5. **Update Docs**: Document the new target in [`README.md`](README.md) and [`skills/icon-toolkit/SKILL.md`](skills/icon-toolkit/SKILL.md).

### Adding a New Image Format

1. **Update Enum**: Add the format variant to `ImageFormat` in [`crates/icon-toolkit-core/src/types.rs`](crates/icon-toolkit-core/src/types.rs).
2. **Implement Encoder/Decoder**: Add encoding/decoding logic in [`crates/icon-toolkit-core/src/converter.rs`](crates/icon-toolkit-core/src/converter.rs).
3. **Add Integration Tests**: Validate conversion to and from the new format in `integration_tests.rs`.

### Adding a New AI Provider

1. **Update Provider Enum**: Add the provider variant to `AiProvider` in [`crates/icon-toolkit-core/src/types.rs`](crates/icon-toolkit-core/src/types.rs).
2. **Implement API Fetcher**: Add the HTTP client handler function in [`crates/icon-toolkit-core/src/ai.rs`](crates/icon-toolkit-core/src/ai.rs).
3. **Update CLI and NAPI Options**: Update `cli/src/main.rs`, `package/src/lib.rs`, and `package/index.ts`.

---

## 📝 Commit Message Guidelines

We follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

- `feat: <description>` — A new feature
- `fix: <description>` — A bug fix
- `docs: <description>` — Documentation changes
- `refactor: <description>` — Code refactoring without behavioral changes
- `test: <description>` — Adding or updating test cases
- `chore: <description>` — Tooling, dependencies, or maintenance tasks

**Example:**
```bash
git commit -m "feat(generator): add watchOS icon set target support"
```

---

## 📬 Submitting a Pull Request

1. Create a descriptive feature branch from `main`:
   ```bash
   git checkout -b feat/my-new-feature
   ```
2. Commit your changes following the commit message guidelines.
3. Run the complete validation checklist:
   ```bash
   # Rust checks
   cargo fmt --check
   cargo clippy --workspace -- -D warnings
   cargo test --workspace

   # TypeScript checks
   bun run check
   bun test
   ```
4. Push your branch to GitHub:
   ```bash
   git push origin feat/my-new-feature
   ```
5. Open a Pull Request on GitHub with a clear description of what changed and why.

Thank you for helping make `icon-toolkit` awesome! 🚀
