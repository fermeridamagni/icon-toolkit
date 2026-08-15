# icontk

> High-performance Rust CLI & TypeScript API for icon set generation, format conversion, and AI icon synthesis.

[![npm version](https://img.shields.io/npm/v/icontk.svg)](https://www.npmjs.com/package/icontk)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

`icontk` is a fast tool written in Rust with pre-compiled native NAPI-RS bindings for Node.js / Bun. It allows developers to generate complete icon sets for Web, Mobile (iOS/Android), and Desktop (macOS/Windows/Linux) from a single source image, convert between image formats, and synthesize icons with AI.

## Installation

### As CLI Tool

```bash
# Using bun
bun add -g icontk

# Using npm / npx
npx icontk --help
```

### As TypeScript / JavaScript Library

```bash
bun add icontk
# or
npm install icontk
```

## CLI Usage

```bash
# Generate icon sets for all platforms (Web, Mobile, Desktop)
icontk generate -i ./icon.png -t all -o ./output/icons

# Convert image format (PNG, WebP, SVG, ICO, JPG)
icontk convert -i ./logo.png -f webp -q 85 -o ./logo.webp

# Generate icon with AI
icontk ai -p "minimalist modern camera icon with clean lines" --provider openai -o ./camera.png
```

## TypeScript API

```typescript
import { generateIcons, convertImage, generateAiIcon } from "icontk";

// 1. Generate icon sets
const result = generateIcons({
  inputPath: "./logo.png",
  targets: ["web", "mobile", "desktop"],
  outputDir: "./output/icons",
  mode: "both", // light & dark mode support
});
console.log(result.summary);

// 2. Convert format
const converted = convertImage({
  inputPath: "./icon.png",
  format: "webp",
  quality: 90,
  outputPath: "./icon.webp",
});
console.log(`Saved ${converted.outputPath} (${converted.fileSize} bytes)`);

// 3. AI Icon Synthesis
const aiResult = await generateAiIcon({
  prompt: "sleek gradient chat bubble icon",
  provider: "openai",
  apiKey: process.env.OPENAI_API_KEY,
  autoGenerateTargets: ["web"],
});
console.log(`Generated: ${aiResult.imagePath}`);
```

## License

GNU General Public License v3.0 (GPL-3.0)
