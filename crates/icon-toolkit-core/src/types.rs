//! Core data structures, configuration options, enums, and result types.
//!
//! Provides types for target selections (`Web`, `Mobile`, `Desktop`), mode selections
//! (`Light`, `Dark`, `Both`), format definitions, AI provider configurations, and options.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Target platforms for icon set generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IconTarget {
    /// Web applications (favicon.ico, apple-touch-icon, android manifest icons).
    Web,
    /// Mobile applications (iOS AppIcon.appiconset & Android mipmap icons).
    Mobile,
    /// Desktop applications (macOS AppIcon.iconset/.icns, Windows ico, Linux hicolor).
    Desktop,
}

impl IconTarget {
    /// Parse target from string representation.
    pub fn from_str_val(val: &str) -> Option<Self> {
        match val.to_lowercase().as_str() {
            "web" => Some(IconTarget::Web),
            "mobile" => Some(IconTarget::Mobile),
            "desktop" => Some(IconTarget::Desktop),
            _ => None,
        }
    }
}

/// Light / Dark theme mode preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    /// Standard light theme icon set.
    #[default]
    Light,
    /// Dark theme icon set.
    Dark,
    /// Generate both light and dark theme icon sets into separate folders.
    Both,
}

impl Mode {
    /// Parse mode from string value.
    pub fn from_str_val(val: &str) -> Self {
        match val.to_lowercase().as_str() {
            "dark" => Mode::Dark,
            "both" => Mode::Both,
            _ => Mode::Light,
        }
    }
}

/// Image formats supported for conversion and encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageFormat {
    Png,
    Webp,
    Svg,
    Ico,
    Jpg,
}

impl ImageFormat {
    /// Parse extension/format string.
    pub fn from_ext(ext: &str) -> Option<Self> {
        let lowered = ext.to_lowercase();
        match lowered.trim_start_matches('.') {
            "png" => Some(ImageFormat::Png),
            "webp" => Some(ImageFormat::Webp),
            "svg" => Some(ImageFormat::Svg),
            "ico" => Some(ImageFormat::Ico),
            "jpg" | "jpeg" => Some(ImageFormat::Jpg),
            _ => None,
        }
    }

    /// Extension string representation.
    pub fn extension(&self) -> &'static str {
        match self {
            ImageFormat::Png => "png",
            ImageFormat::Webp => "webp",
            ImageFormat::Svg => "svg",
            ImageFormat::Ico => "ico",
            ImageFormat::Jpg => "jpg",
        }
    }
}

/// Options for icon set generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IconGeneratorOptions {
    /// Primary source image file path.
    pub input_path: PathBuf,
    /// Secondary source image file path for Dark Mode (optional).
    pub dark_input_path: Option<PathBuf>,
    /// Selected targets to generate (Web, Mobile, Desktop).
    pub targets: Vec<IconTarget>,
    /// Mode selection (Light, Dark, Both).
    pub mode: Mode,
    /// Output base directory.
    pub output_dir: PathBuf,
    /// Padding percentage applied around icon (0 to 40).
    pub padding_percent: u32,
    /// Background color hex (e.g. "#ffffff", "#000000", or "transparent").
    pub background_color: Option<String>,
    /// Border radius percentage (0 to 50, where 50 is circle/rounded pill).
    pub border_radius_percent: u32,
}

impl Default for IconGeneratorOptions {
    fn default() -> Self {
        Self {
            input_path: PathBuf::from("icon.png"),
            dark_input_path: None,
            targets: vec![IconTarget::Web, IconTarget::Mobile, IconTarget::Desktop],
            mode: Mode::Light,
            output_dir: PathBuf::from("output/icons"),
            padding_percent: 0,
            background_color: None,
            border_radius_percent: 0,
        }
    }
}

/// Options for image conversion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertOptions {
    /// Input file path.
    pub input_path: PathBuf,
    /// Output file path (optional; inferred if not provided).
    pub output_path: Option<PathBuf>,
    /// Target image format (PNG, WebP, SVG, ICO, JPG).
    pub format: ImageFormat,
    /// Quality percentage (1 to 100, relevant for WebP/JPG).
    pub quality: u8,
    /// Optional target width.
    pub width: Option<u32>,
    /// Optional target height.
    pub height: Option<u32>,
    /// Optional background color fill for transparent sources.
    pub background_color: Option<String>,
}

/// AI Image Model Providers for Icon Generation (BYOK).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum AiProvider {
    /// OpenAI (DALL-E 3 / gpt-4o).
    #[default]
    OpenAi,
    /// Stability AI (SD3 / Core).
    Stability,
    /// Google Gemini / Imagen 3.
    Gemini,
    /// Generic OpenAI-compatible API endpoint.
    GenericOpenAi,
}

impl AiProvider {
    pub fn from_str_val(val: &str) -> Self {
        match val.to_lowercase().as_str() {
            "stability" => AiProvider::Stability,
            "gemini" => AiProvider::Gemini,
            "generic" | "custom" | "openrouter" | "ollama" => AiProvider::GenericOpenAi,
            _ => AiProvider::OpenAi,
        }
    }
}

/// Configuration options for AI Icon Generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiGeneratorOptions {
    /// Text prompt describing the icon to generate.
    pub prompt: String,
    /// AI service provider.
    pub provider: AiProvider,
    /// API Key (BYOK). If None, environment variable will be used.
    pub api_key: Option<String>,
    /// Custom API endpoint URL (for Generic OpenAI-compatible provider).
    pub endpoint: Option<String>,
    /// Model name (e.g. "dall-e-3", "imagen-3.0-generate-002", etc.).
    pub model: Option<String>,
    /// Icon output size (e.g., 1024, 512).
    pub size: u32,
    /// Quality setting ("standard" or "hd").
    pub quality: String,
    /// Output file path.
    pub output_path: PathBuf,
    /// Automatically trigger target icon set generator after AI image generation.
    pub auto_generate_targets: Option<Vec<IconTarget>>,
}

/// Result metadata for icon generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorResult {
    /// List of created file paths.
    pub created_files: Vec<String>,
    /// Execution log summary.
    pub summary: String,
}

/// Result metadata for image conversion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertResult {
    /// Output file path.
    pub output_path: String,
    /// Format extension.
    pub format: String,
    /// File size in bytes.
    pub file_size: u64,
}

/// Result metadata for AI icon generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiGeneratorResult {
    /// Saved AI image path.
    pub image_path: String,
    /// Generated targets result if auto_generate_targets was set.
    pub targets_result: Option<GeneratorResult>,
}
