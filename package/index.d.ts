/**
 * TypeScript API for `icon-toolkit`
 *
 * Provides native bindings and helper functions to generate icon sets for Web, Mobile,
 * and Desktop, convert images between formats (PNG, WebP, SVG, ICO, JPG), and generate icons with AI.
 */
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
/**
 * Generate icon sets for Web, Mobile, and Desktop platforms.
 */
export declare function generateIcons(options: IconGeneratorOptions): GeneratorResult;
/**
 * Convert an image to a different extension format (PNG, WebP, SVG, ICO, JPG).
 */
export declare function convertImage(options: ConvertOptions): ConvertResult;
/**
 * Generate an icon image using AI (BYOK) and optionally trigger target generation.
 */
export declare function generateAiIcon(options: AiGeneratorOptions): Promise<AiGeneratorResult>;
