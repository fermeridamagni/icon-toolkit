---
name: icon-toolkit
description: Generate icons for Web, Mobile (iOS/Android), and Desktop (macOS/Windows/Linux) from a single source image, convert images between PNG, WebP, SVG, ICO, and JPG, or synthesize icons using AI models (BYOK). Make sure to use this skill whenever the user mentions generating app icons, favicons, iOS/Android icons, macOS/Windows/Linux icons, converting image formats, scaling or padding icons, creating light/dark mode icon sets, or generating AI icons.
---

# Icon Toolkit

`icon-toolkit` is a CLI tool and TypeScript package written in Rust to generate, convert, and synthesize icons from a single source image for Web, Mobile, and Desktop platforms.

## Core Capabilities

1. **Multi-Target Icon Generation (`icon-toolkit generate`)**
   - Single-source asset pipeline for Web (`favicon.ico`, PNGs, `site.webmanifest`), Mobile (`iOS AppIcon.appiconset`, `Android mipmaps`), and Desktop (`macOS .icns`, `Windows .ico`, `Linux hicolor`).
   - Supports light mode, dark mode, or both simultaneously (`--mode both`).
   - Custom padding (`-p, --padding <0..45>`), background color (`-b, --background <HEX>`), and squircle / rounded corners (`-r, --border-radius <0..50>`).

2. **Image Format Conversion (`icon-toolkit convert`)**
   - High-performance image converter between `png`, `webp`, `svg`, `ico`, and `jpg`.
   - Quality control (`-q, --quality <1..100>`), dimension scaling (`-w, --width`, `-H, --height`), and background fill for transparent inputs (`-b, --background`).

3. **AI Icon Synthesis (`icon-toolkit ai`)**
   - Bring-Your-Own-Key (BYOK) AI icon generation supporting OpenAI (DALL-E 3), Stability AI, Gemini (Imagen 3), or generic OpenAI-compatible endpoints.
   - Direct target pipeline integration via `--auto-generate web,mobile,desktop` to immediately produce complete icon sets from a text prompt.

---

## How to Execute `icon-toolkit`

Execute the CLI using the pre-built target binary or `cargo run`:

```bash
# Recommended binary invocation:
./target/release/icon-toolkit <COMMAND> [OPTIONS]
# Or using debug binary / cargo:
cargo run --bin icon-toolkit -- <COMMAND> [OPTIONS]
```

---

## Command Reference & Usage Examples

### 1. Generating Icon Sets (`generate` / `g`)

Generate icon sets for Web, Mobile, and/or Desktop targets.

```bash
# Generate all targets (Web, Mobile, Desktop) with default settings
icon-toolkit generate -i path/to/source.png -o output/icons

# Generate Web and Mobile icons with 5% padding and white background
icon-toolkit generate -i demo-icon.png -t web,mobile -p 5 -b "#ffffff" -o output/icons

# Generate both Light and Dark mode desktop icons with rounded corners (15%)
icon-toolkit generate -i light-icon.png --dark-input dark-icon.png -t desktop -m both -r 15 -o output/icons

# Full targets with dark mode and squircle rounded corners
icon-toolkit generate -i source.png -t all -m both -p 8 -r 20 -b transparent -o output/app-icons
```

**Options:**
- `-i, --input <PATH>`: Primary input image path (PNG, WebP, SVG, JPG) [Required]
- `--dark-input <PATH>`: Secondary input image path for dark theme
- `-t, --target <web,mobile,desktop|all>`: Target platforms (default: `web,mobile,desktop`)
- `-m, --mode <light|dark|both>`: Theme mode selection (default: `light`)
- `-o, --output <DIR>`: Output destination directory (default: `output/icons`)
- `-p, --padding <0..45>`: Padding percentage inside icon frame (default: `0`)
- `-b, --background <HEX>`: Background color hex code (e.g. `#ffffff` or `transparent`)
- `-r, --border-radius <0..50>`: Corner border radius percentage (default: `0`)

---

### 2. Format Conversion (`convert` / `c`)

Convert images to PNG, WebP, SVG, ICO, or JPG formats with optional resizing.

```bash
# Convert PNG to WebP with 85% quality and custom dimensions (256x256)
icon-toolkit convert -i input.png -o output/icon.webp -f webp -q 85 -w 256 -H 256

# Convert raster PNG to SVG vector wrapper
icon-toolkit convert -i input.png -o output/icon.svg -f svg

# Convert transparent PNG to JPG with solid black background
icon-toolkit convert -i input.png -o output/icon.jpg -f jpg -b "#000000"
```

**Options:**
- `-i, --input <PATH>`: Source image file path [Required]
- `-o, --output <PATH>`: Output file destination path (inferred if omitted)
- `-f, --format <png|webp|svg|ico|jpg>`: Target image format (default: `png`)
- `-q, --quality <1..100>`: Compression quality percentage (default: `default`)
- `-w, --width <PX>`: Custom output width in pixels
- `-H, --height <PX>`: Custom output height in pixels
- `-b, --background <HEX>`: Background color hex fill for transparent sources

---

### 3. AI Icon Generation (`ai` / `a`)

Synthesize icons using AI models and optionally auto-generate multi-target asset sets.

```bash
# Synthesize an icon using OpenAI DALL-E 3
icon-toolkit ai -p "Modern 3D gradient rocket icon, minimalist logo, vector style" --provider openai -o output/ai-rocket.png

# Synthesize an icon and immediately auto-generate Web, Mobile, and Desktop icon sets
icon-toolkit ai -p "Futuristic neon shield icon" --provider openai --auto-generate all -o output/ai-shield.png
```

**Options:**
- `-p, --prompt <STRING>`: Text prompt describing the icon concept [Required]
- `--provider <openai|stability|gemini|generic>`: AI provider (default: `openai`)
- `--api-key <KEY>`: API key override (defaults to env vars `OPENAI_API_KEY`, `STABILITY_API_KEY`, etc.)
- `--endpoint <URL>`: Custom API base URL endpoint for OpenAI-compatible providers
- `--model <MODEL>`: Specific AI model ID (e.g., `dall-e-3`, `imagen-3.0-generate-002`)
- `-s, --size <PX>`: Synthesis image resolution (default: `1024`)
- `-q, --quality <standard|hd>`: Quality tier (`standard` or `hd`, default: `standard`)
- `-o, --output <PATH>`: Destination path for synthesized image (default: `output/ai-icon.png`)
- `--auto-generate <web,mobile,desktop|all>`: Automatically trigger `generate` subcommand on synthesized image

---

## Best Practices & Guidelines

1. **High Resolution Source**: Always prefer source images with at least 512x512 (ideally 1024x1024) resolution or SVG for crisp downscaling across platform icons.
2. **Transparent Backgrounds**: When generating app icons for mobile or desktop platforms, ensure background fill (`-b "#ffffff"` or `-b transparent`) and padding (`-p 5..10`) are set appropriately to prevent clipping.
3. **TypeScript Integration**: If building node/bun applications, the `@icon-toolkit/napi` package can be imported directly:
   ```ts
   import { generateIcons, convertImage } from '@icon-toolkit/napi';
   ```
