# 🎨 Icon Toolkit (`icon-toolkit`)

[![License: GPL-3.0](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-2021_Edition-orange.svg)](https://www.rust-lang.org/)
[![Bun](https://img.shields.io/badge/Runtime-Bun-black.svg)](https://bun.sh/)
[![TypeScript](https://img.shields.io/badge/TypeScript-Strict-blue.svg)](https://www.typescriptlang.org/)
[![Ultracite](https://img.shields.io/badge/Linter-Ultracite-8A2BE2.svg)](https://github.com/biomejs/biome)

A high-performance CLI tool and TypeScript library written in Rust to **generate**, **convert**, and **AI-synthesize** icons from a single source for **Web**, **Mobile** (iOS & Android), and **Desktop** (macOS, Windows, & Linux) platforms.

---

## 📑 Table of Contents

- [Features](#-features)
- [Architecture & Workspace](#-architecture--workspace)
- [Installation](#-installation)
  - [Prerequisites](#prerequisites)
  - [Build from Source (Rust CLI)](#build-from-source-rust-cli)
  - [Build TypeScript / NAPI Native Package](#build-typescript--napi-native-package)
- [CLI Reference & Usage](#-cli-reference--usage)
  - [1. Generate Icon Sets (`generate` / `g`)](#1-generate-icon-sets-generate--g)
  - [2. Convert Formats (`convert` / `c`)](#2-convert-formats-convert--c)
  - [3. AI Icon Synthesis (`ai` / `a`)](#3-ai-icon-synthesis-ai--a)
- [TypeScript API Reference](#-typescript-api-reference)
  - [`generateIcons(options)`](#generateiconsoptions)
  - [`convertImage(options)`](#convertimageoptions)
  - [`generateAiIcon(options)`](#generateaiiconoptions)
- [Target Platform Specifications](#-target-platform-specifications)
- [Agent Skill Integration](#-agent-skill-integration)
- [Running Tests & Quality Checks](#-running-tests--quality-checks)
- [License](#-license)

---

## ✨ Features

- **🚀 Single-Source Multi-Platform Icon Sets**: Transform a single high-resolution image (`PNG`, `WebP`, `SVG`, `JPG`) into complete production-ready icon assets for:
  - **Web**: `favicon.ico`, `favicon-16x16.png`, `favicon-32x32.png`, `apple-touch-icon.png`, `android-chrome-192x192.png`, `android-chrome-512x512.png`, and `site.webmanifest`.
  - **iOS**: Complete `AppIcon.appiconset` with all standard `@2x`, `@3x`, and `1024x1024` App Store icons, plus auto-generated `Contents.json`.
  - **Android**: Legacy & adaptive/round mipmaps (`mdpi`, `hdpi`, `xhdpi`, `xxhdpi`, `xxxhdpi`) into standard `res/mipmap-*` directory structures.
  - **macOS**: Apple `AppIcon.iconset` with 16px to 512px (@2x) resolutions.
  - **Windows**: Multi-resolution `icon.ico` embedding 16, 24, 32, 48, 64, 128, and 256px frames.
  - **Linux**: Freedesktop `hicolor` hierarchy (16x16 up to 512x512).
- **🌓 Light & Dark Theme Variants**: Generate light mode, dark mode, or both simultaneously (`--mode both`) with dedicated dark source inputs (`--dark-input`).
- **🎨 Custom Padding, Backgrounds & Rounded Corners**:
  - Inner canvas padding percentage (`-p, --padding <0..45>`).
  - Solid background color fills with hex codes (`-b, --background "#ffffff"` or `transparent`).
  - Squircle / rounded pill masking (`-r, --border-radius <0..50>`).
- **🔄 High-Performance Image Converter**:
  - Convert between `PNG`, `WebP`, `SVG`, `ICO`, and `JPG`.
  - Direct SVG vector wrapping and high-fidelity SVG rasterization via `resvg` and `tiny-skia`.
  - Quality compression control (`-q, --quality <1..100>`) and custom dimension scaling (`-w, --width`, `-H, --height`).
- **🤖 AI Icon Generation (BYOK - Bring Your Own Key)**:
  - Synthesize icons directly via text prompts with OpenAI (DALL-E 3), Stability AI (SD3/Core), Google Gemini (Imagen 3), or any OpenAI-compatible custom endpoint.
  - One-shot pipeline: Synthesize an icon and immediately trigger target icon set generation via `--auto-generate web,mobile,desktop`.
- **⚡ Dual Interface**: Fast standalone native CLI binary and TypeScript package with zero-overhead NAPI-RS bindings.

---

## 🏗 Architecture & Workspace

The project is structured as a modular Cargo workspace and Bun/TypeScript package:

```txt
.
├── cli/                          # Native Rust CLI binary (`icon-toolkit`)
│   ├── Cargo.toml
│   └── src/main.rs
├── crates/
│   └── icon-toolkit-core/        # Pure Rust core processing engine
│       ├── Cargo.toml
│       ├── src/
│       │   ├── lib.rs            # Core exports
│       │   ├── generator.rs      # Multi-target icon generator & manifests
│       │   ├── converter.rs      # Format converter & SVG/ICO encoders
│       │   ├── ai.rs             # BYOK AI provider clients
│       │   ├── types.rs          # Data types and configuration options
│       │   └── error.rs          # Custom error definitions
│       └── tests/
│           └── integration_tests.rs # Rust integration test suite
├── package/                      # TypeScript package with NAPI-RS bindings
│   ├── Cargo.toml                # NAPI-RS native cdylib crate
│   ├── package.json
│   ├── src/lib.rs                # NAPI-RS JS/Rust bridge
│   ├── index.ts                  # TypeScript API entry point & loader
│   └── cli.ts                    # Bun-executable CLI wrapper
├── skills/                       # Built-in Agent Skill for Antigravity & LLMs
│   └── icon-toolkit/
│       ├── SKILL.md              # Skill instructions and reference
│       └── evals/evals.json      # LLM evaluation triggers
├── tests/
│   └── ts-package.test.ts        # Bun test suite for TypeScript API
├── biome.jsonc                   # Ultracite / Biome linter configuration
└── Cargo.toml                    # Root Cargo workspace manifest
```

---

## 📦 Installation

### Prerequisites

- [Rust & Cargo](https://rustup.rs/) (v1.75+ / 2021 Edition)
- [Bun](https://bun.sh/) (v1.3+)

### Build from Source (Rust CLI)

```bash
# Clone repository
git clone https://github.com/your-username/icon-toolkit.git
cd icon-toolkit

# Build release binary
cargo build --release --workspace

# The binary is available at ./target/release/icon-toolkit
./target/release/icon-toolkit --help
```

To install the binary globally to your Cargo path:

```bash
cargo install --path cli
```

### Build TypeScript / NAPI Native Package

```bash
# Install dependencies
bun install

# Build NAPI native bindings in release mode
cd package
bun run build
cd ..

# Run TypeScript tests
bun test
```

---

## 💻 CLI Reference & Usage

### 1. Generate Icon Sets (`generate` / `g`)

Generate comprehensive icon sets for Web, Mobile, and Desktop platforms from a single image.

```bash
# Basic usage: Generate all platform targets with defaults
icon-toolkit generate -i icon.png -o output/icons

# Target specific platforms (web and mobile only)
icon-toolkit generate -i icon.png -t web,mobile -o output/icons

# Add 5% inner padding and a white background fill
icon-toolkit generate -i icon.png -p 5 -b "#ffffff" -o output/icons

# Apply 15% rounded corner radius (squircle)
icon-toolkit generate -i icon.png -r 15 -o output/icons

# Generate both Light and Dark mode sets using separate source assets
icon-toolkit generate \
  -i icon-light.png \
  --dark-input icon-dark.png \
  -t all \
  -m both \
  -p 8 \
  -r 12 \
  -o output/app-icons
```

#### Options (`generate`)

| Option | Shorthand | Description | Default |
|---|---|---|---|
| `--input <PATH>` | `-i` | Primary input image path (`PNG`, `WebP`, `SVG`, `JPG`) **[Required]** | — |
| `--dark-input <PATH>` | | Secondary input image path for Dark Mode | — |
| `--target <TARGETS>` | `-t` | Comma-separated targets: `web`, `mobile`, `desktop`, or `all` | `web,mobile,desktop` |
| `--mode <MODE>` | `-m` | Theme mode: `light`, `dark`, or `both` | `light` |
| `--output <DIR>` | `-o` | Output directory destination | `output/icons` |
| `--padding <0..45>` | `-p` | Canvas inner padding percentage | `0` |
| `--background <HEX>` | `-b` | Background color hex (e.g. `#ffffff` or `transparent`) | `None` |
| `--border-radius <0..50>` | `-r` | Border radius percentage (50 = circular/pill) | `0` |

---

### 2. Convert Formats (`convert` / `c`)

Convert images across `PNG`, `WebP`, `SVG`, `ICO`, and `JPG` formats with optional resizing and quality optimization.

```bash
# Convert PNG to WebP with 85% quality and custom dimensions
icon-toolkit convert -i icon.png -o output/icon.webp -f webp -q 85 -w 256 -H 256

# Convert raster PNG to SVG vector wrapper
icon-toolkit convert -i icon.png -o output/icon.svg -f svg

# Render SVG vector to high-res 1024x1024 PNG
icon-toolkit convert -i logo.svg -o output/logo.png -f png -w 1024 -H 1024

# Convert transparent image to multi-size Windows ICO
icon-toolkit convert -i icon.png -o output/app.ico -f ico

# Convert transparent PNG to JPG with black background
icon-toolkit convert -i icon.png -o output/icon.jpg -f jpg -b "#000000"
```

#### Options (`convert`)

| Option | Shorthand | Description | Default |
|---|---|---|---|
| `--input <PATH>` | `-i` | Source image file path **[Required]** | — |
| `--output <PATH>` | `-o` | Destination file path (inferred from format if omitted) | Inferred |
| `--format <FMT>` | `-f` | Output format: `png`, `webp`, `svg`, `ico`, `jpg` | `png` |
| `--quality <1..100>` | `-q` | Compression quality percentage (WebP/JPG) | `90` |
| `--width <PX>` | `-w` | Target output width in pixels | Source width |
| `--height <PX>` | `-H` | Target output height in pixels | Source height |
| `--background <HEX>` | `-b` | Background hex color for transparent inputs | `None` |

---

### 3. AI Icon Synthesis (`ai` / `a`)

Synthesize app icons using AI image models via Bring-Your-Own-Key (BYOK) and optionally auto-generate multi-target asset pipelines in one step.

```bash
# Generate an icon using OpenAI DALL-E 3 (requires OPENAI_API_KEY)
icon-toolkit ai \
  -p "Minimalist geometric origami bird logo, vibrant cyan and purple gradient, 3d render" \
  --provider openai \
  -o output/ai-bird.png

# Generate an icon with Stability AI (requires STABILITY_API_KEY)
icon-toolkit ai \
  -p "Neon cybernetic shield app icon, dark background" \
  --provider stability \
  -o output/ai-shield.png

# Generate with Google Gemini Imagen 3 (requires GEMINI_API_KEY)
icon-toolkit ai \
  -p "Flat modern rocket icon with smooth shadows" \
  --provider gemini \
  -o output/ai-rocket.png

# Synthesize an icon and immediately auto-generate Web, Mobile, and Desktop icon sets
icon-toolkit ai \
  -p "Sleek finance wallet icon, glassmorphism, golden accents" \
  --provider openai \
  --auto-generate all \
  -o output/wallet.png
```

#### Supported AI Providers & Environment Variables

| Provider | `--provider` | Default Model | Environment Variable |
|---|---|---|---|
| **OpenAI** | `openai` | `dall-e-3` | `OPENAI_API_KEY` |
| **Stability AI** | `stability` | `core` | `STABILITY_API_KEY` |
| **Google Gemini** | `gemini` | `imagen-3.0-generate-002` | `GEMINI_API_KEY` |
| **Generic OpenAI** | `generic` | Custom | `--api-key` / `OPENAI_API_KEY` |

#### Options (`ai`)

| Option | Shorthand | Description | Default |
|---|---|---|---|
| `--prompt <PROMPT>` | `-p` | Text prompt describing the icon concept **[Required]** | — |
| `--provider <PROV>` | | AI provider: `openai`, `stability`, `gemini`, `generic` | `openai` |
| `--api-key <KEY>` | | API key override (overrides environment variable) | Env var |
| `--endpoint <URL>` | | Custom endpoint URL for generic OpenAI-compatible APIs | OpenAI URL |
| `--model <MODEL>` | | Specific model name | Provider default |
| `--size <PX>` | `-s` | Output resolution in pixels (e.g. `1024`, `512`) | `1024` |
| `--quality <TIER>` | `-q` | Quality tier (`standard` or `hd`) | `standard` |
| `--output <PATH>` | `-o` | Destination file path for generated AI image | `output/ai-icon.png` |
| `--auto-generate <TARGETS>` | | Automatically trigger icon set generation (`web,mobile,desktop` or `all`) | `None` |

---

## 🔌 TypeScript API Reference

The TypeScript package exposes native high-speed bindings via NAPI-RS.

```ts
import { generateIcons, convertImage, generateAiIcon } from "icon-toolkit";
```

### `generateIcons(options)`

Generates complete icon sets for specified platforms.

```ts
import { generateIcons } from "icon-toolkit";

const result = generateIcons({
  inputPath: "source-icon.png",
  darkInputPath: "source-dark.png", // Optional
  targets: ["web", "mobile", "desktop"],
  mode: "both", // 'light' | 'dark' | 'both'
  outputDir: "output/app-icons",
  paddingPercent: 5,
  backgroundColor: "#ffffff",
  borderRadiusPercent: 10,
});

console.log(result.summary);
console.log(`Generated ${result.createdFiles.length} files:`, result.createdFiles);
```

### `convertImage(options)`

Converts image formats with optional dimensions and quality settings.

```ts
import { convertImage } from "icon-toolkit";

const result = convertImage({
  inputPath: "icon.png",
  outputPath: "output/icon.webp",
  format: "webp", // 'png' | 'webp' | 'svg' | 'ico' | 'jpg'
  quality: 85,
  width: 512,
  height: 512,
});

console.log(`Saved ${result.format.toUpperCase()} to ${result.outputPath} (${result.fileSize} bytes)`);
```

### `generateAiIcon(options)`

Synthesizes an icon using AI models and optionally triggers target asset generation.

```ts
import { generateAiIcon } from "icon-toolkit";

const result = await generateAiIcon({
  prompt: "Minimalist glowing folder icon, modern SaaS style",
  provider: "openai",
  apiKey: process.env.OPENAI_API_KEY,
  size: 1024,
  quality: "standard",
  outputPath: "output/ai-folder.png",
  autoGenerateTargets: ["web", "mobile", "desktop"],
});

console.log(`AI Image created at: ${result.imagePath}`);
if (result.targetsSummary) {
  console.log(`Target pipeline: ${result.targetsSummary}`);
}
```

---

## 🎯 Target Platform Specifications

When generating icon sets (`generate`), `icon-toolkit` creates the following platform-compliant assets:

### Web Target (`web/`)
| File | Dimensions | Purpose |
|---|---|---|
| `favicon.ico` | Multi-size (16, 32, 48) | Legacy and desktop browser favicons |
| `favicon-16x16.png` | 16x16 | Classic browser tabs |
| `favicon-32x32.png` | 32x32 | Standard browser tabs |
| `apple-touch-icon.png` | 180x180 | iOS Safari bookmarks & Home Screen |
| `android-chrome-192x192.png` | 192x192 | Android PWA icon |
| `android-chrome-512x512.png` | 512x512 | Android PWA splash icon |
| `site.webmanifest` | JSON | PWA Web App Manifest file |

### Mobile Target (`mobile/`)
| Platform | Destination Path | Specifications |
|---|---|---|
| **iOS** | `ios/AppIcon.appiconset/` | `Icon-20@2x`, `Icon-20@3x`, `Icon-29@2x`, `Icon-29@3x`, `Icon-40@2x`, `Icon-40@3x`, `Icon-60@2x`, `Icon-60@3x`, `Icon-76@2x`, `Icon-83.5@2x`, `Icon-1024.png`, and valid Xcode `Contents.json` |
| **Android** | `android/res/mipmap-*` | Standard and rounded icons for `mipmap-mdpi` (48px), `mipmap-hdpi` (72px), `mipmap-xhdpi` (96px), `mipmap-xxhdpi` (144px), `mipmap-xxxhdpi` (192px) |

### Desktop Target (`desktop/`)
| OS | Destination Path | Specifications |
|---|---|---|
| **macOS** | `macos/AppIcon.iconset/` | `icon_16x16.png` through `icon_512x512@2x.png` (1024px) for Apple `iconutil` |
| **Windows** | `windows/icon.ico` | Multi-resolution embedded icon (16, 24, 32, 48, 64, 128, 256px) |
| **Linux** | `linux/hicolor/<SIZE>/apps/app-icon.png` | Standard Freedesktop sizes (16, 32, 48, 64, 128, 256, 512px) |

---

## 🤖 Agent Skill Integration

`icon-toolkit` includes a built-in Agent Skill for Google Antigravity and AI coding assistants located at [`skills/icon-toolkit/SKILL.md`](skills/icon-toolkit/SKILL.md).

When using Antigravity, the skill is automatically discovered and used whenever you ask the agent to:
- Generate application icons for Web, Mobile, or Desktop.
- Convert image assets across formats or resize icons.
- Generate icons from natural language prompts using AI models.

---

## 🧪 Running Tests & Quality Checks

Ensure code quality and test coverage with the following commands:

```bash
# 1. Run all Rust unit and integration tests
cargo test --workspace

# 2. Check Rust formatting & Clippy linter
cargo fmt --check
cargo clippy --workspace -- -D warnings

# 3. Check TypeScript formatting & linting with Ultracite (Biome)
bun run check

# 4. Automatically fix TypeScript formatting & linting issues
bun run fix

# 5. Run TypeScript API test suite with Bun
bun test
```

---

## 📄 License

This project is licensed under the **GNU General Public License v3.0** (GPL-3.0). See the [LICENSE](LICENSE) file for details.
