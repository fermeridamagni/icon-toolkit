# Project Guidelines

This project is a CLI tool written in Rust, designed to provide a seamless experience when creating and managing icons for various purposes (like generating icons for a web, mobile or desktop applications from a single source).

## Rules

- Document and explain why the code is for.
- Get pre-indexed knowledge about the project using the Codegraph MCP.
- Always use up-to-date info with the Context7 MCP or searching the web.
- Always use Bun as the package manager and runtime environment instead of Node.js.
- Always use TypeScript instead of Javascript.
- Always use Ultracite (Biome's zero-config preset) for TypeScript code formatting and linting.
  - Most issues are automatically fixable with `bun run fix`.
  - Before start writing a `ts` or `tsx` file, check the [Ultracite Code Standards](ULTRACITE.md).

## Architecture

The project follows a modular architecture pattern, with clear separation of concerns. Each module should have its own directory and contain related components, services, and utilities.

```txt
.
├── docs/                         # Documentation Page (Astro v7 - Starlight)
├── skills/                       # Built-in Agent Skills
├── package/                      # Typescript package source code
├── crates/                       # Rust crates source code
├── cli/                          # Rust CLI tool source code
├── tests/                        # Test cases for the project
```
