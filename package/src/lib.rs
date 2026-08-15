//! NAPI-RS Node.js / Bun bindings for `icon-toolkit`.
//!
//! Exposes native functions `generateIcons`, `convertImage`, and `generateAiIcon` to TypeScript/JavaScript.

#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::path::PathBuf;

use icon_toolkit_core::{
    self,
    types::{
        AiGeneratorOptions, AiProvider, ConvertOptions, IconGeneratorOptions, IconTarget,
        ImageFormat, Mode,
    },
};

#[napi(object)]
pub struct JsIconGeneratorOptions {
    pub input_path: String,
    pub dark_input_path: Option<String>,
    pub targets: Option<Vec<String>>,
    pub mode: Option<String>,
    pub output_dir: Option<String>,
    pub padding_percent: Option<u32>,
    pub background_color: Option<String>,
    pub border_radius_percent: Option<u32>,
}

#[napi(object)]
pub struct JsGeneratorResult {
    pub created_files: Vec<String>,
    pub summary: String,
}

#[napi(object)]
pub struct JsConvertOptions {
    pub input_path: String,
    pub output_path: Option<String>,
    pub format: String,
    pub quality: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub background_color: Option<String>,
}

#[napi(object)]
pub struct JsConvertResult {
    pub output_path: String,
    pub format: String,
    pub file_size: u32,
}

#[napi(object)]
pub struct JsAiGeneratorOptions {
    pub prompt: String,
    pub provider: Option<String>,
    pub api_key: Option<String>,
    pub endpoint: Option<String>,
    pub model: Option<String>,
    pub size: Option<u32>,
    pub quality: Option<String>,
    pub output_path: Option<String>,
    pub auto_generate_targets: Option<Vec<String>>,
}

#[napi(object)]
pub struct JsAiGeneratorResult {
    pub image_path: String,
    pub targets_summary: Option<String>,
}

/// Native binding for generating icon sets.
#[napi]
pub fn generate_icons(options: JsIconGeneratorOptions) -> Result<JsGeneratorResult> {
    let targets = if let Some(t_list) = options.targets {
        t_list
            .iter()
            .filter_map(|t| IconTarget::from_str_val(t))
            .collect()
    } else {
        vec![IconTarget::Web, IconTarget::Mobile, IconTarget::Desktop]
    };

    let opts = IconGeneratorOptions {
        input_path: PathBuf::from(options.input_path),
        dark_input_path: options.dark_input_path.map(PathBuf::from),
        targets,
        mode: Mode::from_str_val(options.mode.as_deref().unwrap_or("light")),
        output_dir: PathBuf::from(options.output_dir.unwrap_or_else(|| "output/icons".to_string())),
        padding_percent: options.padding_percent.unwrap_or(0),
        background_color: options.background_color,
        border_radius_percent: options.border_radius_percent.unwrap_or(0),
    };

    let res = icon_toolkit_core::generate_icons(&opts)
        .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

    Ok(JsGeneratorResult {
        created_files: res.created_files,
        summary: res.summary,
    })
}

/// Native binding for image format conversion.
#[napi]
pub fn convert_image(options: JsConvertOptions) -> Result<JsConvertResult> {
    let format = ImageFormat::from_ext(&options.format).ok_or_else(|| {
        Error::new(
            Status::InvalidArg,
            format!("Unsupported format: {}", options.format),
        )
    })?;

    let opts = ConvertOptions {
        input_path: PathBuf::from(options.input_path),
        output_path: options.output_path.map(PathBuf::from),
        format,
        quality: options.quality.unwrap_or(90) as u8,
        width: options.width,
        height: options.height,
        background_color: options.background_color,
    };

    let res = icon_toolkit_core::convert_image(&opts)
        .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

    Ok(JsConvertResult {
        output_path: res.output_path,
        format: res.format,
        file_size: res.file_size as u32,
    })
}

/// Native binding for AI icon synthesis.
#[napi]
pub async fn generate_ai_icon(options: JsAiGeneratorOptions) -> Result<JsAiGeneratorResult> {
    let provider = AiProvider::from_str_val(options.provider.as_deref().unwrap_or("openai"));
    let targets = options.auto_generate_targets.map(|t_list| {
        t_list
            .iter()
            .filter_map(|t| IconTarget::from_str_val(t))
            .collect()
    });

    let opts = AiGeneratorOptions {
        prompt: options.prompt,
        provider,
        api_key: options.api_key,
        endpoint: options.endpoint,
        model: options.model,
        size: options.size.unwrap_or(1024),
        quality: options.quality.unwrap_or_else(|| "standard".to_string()),
        output_path: PathBuf::from(options.output_path.unwrap_or_else(|| "output/ai-icon.png".to_string())),
        auto_generate_targets: targets,
    };

    let res = icon_toolkit_core::generate_ai_icon(&opts)
        .await
        .map_err(|e| Error::new(Status::GenericFailure, e.to_string()))?;

    Ok(JsAiGeneratorResult {
        image_path: res.image_path,
        targets_summary: res.targets_result.map(|r| r.summary),
    })
}
