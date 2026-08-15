/**
 * TypeScript API for `icon-toolkit`
 *
 * Provides native bindings and helper functions to generate icon sets for Web, Mobile,
 * and Desktop, convert images between formats (PNG, WebP, SVG, ICO, JPG), and generate icons with AI.
 */

import { existsSync } from "node:fs";
import { join } from "node:path";

export interface IconGeneratorOptions {
  /** Background color hex (e.g. '#ffffff' or 'transparent') */
  backgroundColor?: string;
  /** Border radius percentage (0-50) */
  borderRadiusPercent?: number;
  /** Optional dark mode input image path */
  darkInputPath?: string;
  /** Primary input image path */
  inputPath: string;
  /** Theme mode: 'light' | 'dark' | 'both' */
  mode?: "light" | "dark" | "both";
  /** Base output directory */
  outputDir?: string;
  /** Padding percentage inside icon canvas (0-45) */
  paddingPercent?: number;
  /** Target platforms: 'web' | 'mobile' | 'desktop' */
  targets?: Array<"web" | "mobile" | "desktop" | "all">;
}

export interface GeneratorResult {
  /** Array of generated file paths */
  createdFiles: string[];
  /** Summary message */
  summary: string;
}

export interface ConvertOptions {
  /** Background color hex for transparency */
  backgroundColor?: string;
  /** Output format: 'png' | 'webp' | 'svg' | 'ico' | 'jpg' */
  format: "png" | "webp" | "svg" | "ico" | "jpg";
  /** Custom height in pixels */
  height?: number;
  /** Source image file path */
  inputPath: string;
  /** Destination output path */
  outputPath?: string;
  /** Quality percentage (1-100) */
  quality?: number;
  /** Custom width in pixels */
  width?: number;
}

export interface ConvertResult {
  /** File size in bytes */
  fileSize: number;
  /** Target format extension */
  format: string;
  /** Output file path */
  outputPath: string;
}

export interface AiGeneratorOptions {
  /** API key (BYOK) */
  apiKey?: string;
  /** Auto trigger target generation on output image */
  autoGenerateTargets?: Array<"web" | "mobile" | "desktop">;
  /** Custom API endpoint URL */
  endpoint?: string;
  /** AI Model name */
  model?: string;
  /** Output path for generated image */
  outputPath?: string;
  /** Prompt describing the icon concept */
  prompt: string;
  /** AI provider: 'openai' | 'stability' | 'gemini' | 'generic' */
  provider?: "openai" | "stability" | "gemini" | "generic";
  /** Quality setting ('standard' | 'hd') */
  quality?: "standard" | "hd";
  /** Icon image dimension in pixels */
  size?: number;
}

export interface AiGeneratorResult {
  /** Generated image file path */
  imagePath: string;
  /** Target generation summary if autoGenerateTargets was set */
  targetsSummary?: string;
}

interface NativeBinding {
  convertImage: (options: {
    input_path: string;
    output_path?: string;
    format: string;
    quality?: number;
    width?: number;
    height?: number;
    background_color?: string;
  }) => ConvertResult;
  generateAiIcon: (options: {
    prompt: string;
    provider?: string;
    api_key?: string;
    endpoint?: string;
    model?: string;
    size?: number;
    quality?: string;
    output_path?: string;
    auto_generate_targets?: string[];
  }) => Promise<AiGeneratorResult>;
  generateIcons: (options: {
    input_path: string;
    dark_input_path?: string;
    targets?: string[];
    mode?: string;
    output_dir?: string;
    padding_percent?: number;
    background_color?: string;
    border_radius_percent?: number;
  }) => GeneratorResult;
}

let nativeBinding: NativeBinding | null = null;

function loadNativeBinding(): NativeBinding {
  if (nativeBinding) {
    return nativeBinding;
  }

  const possiblePaths = [
    join(import.meta.dirname, "icon-toolkit-napi.node"),
    join(import.meta.dirname, "icon-toolkit-napi.darwin-arm64.node"),
    join(import.meta.dirname, "icon-toolkit-napi.darwin-x64.node"),
    join(import.meta.dirname, "icon-toolkit-napi.linux-x64-gnu.node"),
    join(import.meta.dirname, "icon-toolkit-napi.win32-x64-msvc.node"),
    join(import.meta.dirname, "index.node"),
    join(import.meta.dirname, "..", "icon-toolkit-napi.node"),
    join(
      import.meta.dirname,
      "..",
      "target",
      "release",
      "libicon_toolkit_napi.dylib"
    ),
    join(
      import.meta.dirname,
      "..",
      "target",
      "release",
      "libicon_toolkit_napi.so"
    ),
    join(
      import.meta.dirname,
      "..",
      "target",
      "release",
      "icon_toolkit_napi.dll"
    ),
    join(
      import.meta.dirname,
      "..",
      "target",
      "debug",
      "libicon_toolkit_napi.dylib"
    ),
  ];

  for (const p of possiblePaths) {
    if (existsSync(p)) {
      try {
        nativeBinding = require(p);
        if (nativeBinding) {
          return nativeBinding;
        }
      } catch {
        // try next path
      }
    }
  }

  try {
    nativeBinding = require("./index.node");
    return nativeBinding as NativeBinding;
  } catch (e) {
    throw new Error(
      "Failed to load icon-toolkit native binary. Please run 'bun run build' inside package/ directory.",
      { cause: e }
    );
  }
}

/**
 * Generate icon sets for Web, Mobile, and Desktop platforms.
 */
export function generateIcons(options: IconGeneratorOptions): GeneratorResult {
  const binding = loadNativeBinding();
  return binding.generateIcons({
    backgroundColor: options.backgroundColor,
    borderRadiusPercent: options.borderRadiusPercent,
    darkInputPath: options.darkInputPath,
    inputPath: options.inputPath,
    mode: options.mode,
    outputDir: options.outputDir,
    paddingPercent: options.paddingPercent,
    targets: options.targets,
  } as unknown as Parameters<typeof binding.generateIcons>[0]);
}

/**
 * Convert an image to a different extension format (PNG, WebP, SVG, ICO, JPG).
 */
export function convertImage(options: ConvertOptions): ConvertResult {
  const binding = loadNativeBinding();
  return binding.convertImage({
    backgroundColor: options.backgroundColor,
    format: options.format,
    height: options.height,
    inputPath: options.inputPath,
    outputPath: options.outputPath,
    quality: options.quality,
    width: options.width,
  } as unknown as Parameters<typeof binding.convertImage>[0]);
}

/**
 * Generate an icon image using AI (BYOK) and optionally trigger target generation.
 */
export async function generateAiIcon(
  options: AiGeneratorOptions
): Promise<AiGeneratorResult> {
  const binding = loadNativeBinding();
  return await binding.generateAiIcon({
    apiKey: options.apiKey,
    autoGenerateTargets: options.autoGenerateTargets,
    endpoint: options.endpoint,
    model: options.model,
    outputPath: options.outputPath,
    prompt: options.prompt,
    provider: options.provider,
    quality: options.quality,
    size: options.size,
  } as unknown as Parameters<typeof binding.generateAiIcon>[0]);
}
