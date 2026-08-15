//! AI Icon Generator module using BYOK (Bring Your Own Key).
//!
//! Connects to AI image synthesis providers (OpenAI DALL-E, Stability AI, Gemini Imagen 3,
//! or custom OpenAI-compatible endpoints), generates an optimized app icon image,
//! and optionally runs the icon set generator pipeline.

use crate::error::{IconToolkitError, Result};
use crate::generator::generate_icons;
use crate::types::{
    AiGeneratorOptions, AiGeneratorResult, AiProvider, IconGeneratorOptions, Mode,
};
use base64::Engine;
use reqwest::Client;
use serde_json::{json, Value};
use std::env;
use std::fs::{self, File};
use std::io::Write;

/// Generate an icon using AI models and save to disk.
pub async fn generate_ai_icon(options: &AiGeneratorOptions) -> Result<AiGeneratorResult> {
    let client = Client::new();

    // Enhance prompt for icon design aesthetics
    let enhanced_prompt = format!(
        "{}, modern app icon, clean vector design, isolated on neutral background, crisp details, 8k",
        options.prompt
    );

    let image_bytes = match options.provider {
        AiProvider::OpenAi => {
            fetch_openai_image(
                &client,
                &enhanced_prompt,
                options.api_key.as_deref(),
                options.model.as_deref().unwrap_or("dall-e-3"),
                options.size,
                &options.quality,
            )
            .await?
        }
        AiProvider::Stability => {
            fetch_stability_image(
                &client,
                &enhanced_prompt,
                options.api_key.as_deref(),
                options.size,
            )
            .await?
        }
        AiProvider::Gemini => {
            fetch_gemini_image(
                &client,
                &enhanced_prompt,
                options.api_key.as_deref(),
                options.model.as_deref().unwrap_or("imagen-3.0-generate-002"),
            )
            .await?
        }
        AiProvider::GenericOpenAi => {
            let endpoint = options
                .endpoint
                .as_deref()
                .unwrap_or("https://api.openai.com/v1/images/generations");
            fetch_generic_openai_image(
                &client,
                endpoint,
                &enhanced_prompt,
                options.api_key.as_deref(),
                options.model.as_deref().unwrap_or("dall-e-3"),
                options.size,
            )
            .await?
        }
    };

    // Ensure parent directory exists
    if let Some(parent) = options.output_path.parent() {
        fs::create_dir_all(parent).map_err(|e| IconToolkitError::Io {
            path: parent.display().to_string(),
            source: e,
        })?;
    }

    // Save image bytes to disk
    let mut file = File::create(&options.output_path).map_err(|e| IconToolkitError::Io {
        path: options.output_path.display().to_string(),
        source: e,
    })?;

    file.write_all(&image_bytes)
        .map_err(|e| IconToolkitError::Io {
            path: options.output_path.display().to_string(),
            source: e,
        })?;

    let image_path_str = options.output_path.display().to_string();

    // Trigger target generation pipeline if requested
    let targets_result = if let Some(targets) = &options.auto_generate_targets {
        let gen_opts = IconGeneratorOptions {
            input_path: options.output_path.clone(),
            dark_input_path: None,
            targets: targets.clone(),
            mode: Mode::Light,
            output_dir: options
                .output_path
                .parent()
                .unwrap_or_else(|| std::path::Path::new("."))
                .join("targets"),
            padding_percent: 0,
            background_color: None,
            border_radius_percent: 0,
        };
        Some(generate_icons(&gen_opts)?)
    } else {
        None
    };

    Ok(AiGeneratorResult {
        image_path: image_path_str,
        targets_result,
    })
}

/// Fetch AI generated image from OpenAI API.
async fn fetch_openai_image(
    client: &Client,
    prompt: &str,
    api_key: Option<&str>,
    model: &str,
    size: u32,
    quality: &str,
) -> Result<Vec<u8>> {
    let key = api_key
        .map(|s| s.to_string())
        .or_else(|| env::var("OPENAI_API_KEY").ok())
        .ok_or_else(|| {
            IconToolkitError::InvalidConfig(
                "OpenAI API key missing. Pass --api-key or set OPENAI_API_KEY environment variable."
                    .to_string(),
            )
        })?;

    let size_str = format!("{}x{}", size.max(1024), size.max(1024));

    let body = json!({
        "model": model,
        "prompt": prompt,
        "n": 1,
        "size": size_str,
        "quality": quality,
        "response_format": "b64_json"
    });

    let res = client
        .post("https://api.openai.com/v1/images/generations")
        .header("Authorization", format!("Bearer {}", key))
        .json(&body)
        .send()
        .await
        .map_err(|e| IconToolkitError::AiService {
            provider: "OpenAI".to_string(),
            message: e.to_string(),
        })?;

    if !res.status().is_success() {
        let err_text = res.text().await.unwrap_or_default();
        return Err(IconToolkitError::AiService {
            provider: "OpenAI".to_string(),
            message: format!("API returned HTTP error: {}", err_text),
        });
    }

    let json_res: Value = res.json().await.map_err(|e| IconToolkitError::AiService {
        provider: "OpenAI".to_string(),
        message: format!("Failed to parse response JSON: {}", e),
    })?;

    let b64 = json_res["data"][0]["b64_json"]
        .as_str()
        .ok_or_else(|| IconToolkitError::AiService {
            provider: "OpenAI".to_string(),
            message: "Response did not contain b64_json image payload".to_string(),
        })?;

    base64::engine::general_purpose::STANDARD
        .decode(b64)
        .map_err(|e| IconToolkitError::AiService {
            provider: "OpenAI".to_string(),
            message: format!("Failed to decode base64 image data: {}", e),
        })
}

/// Fetch AI generated image from Stability AI API.
async fn fetch_stability_image(
    client: &Client,
    prompt: &str,
    api_key: Option<&str>,
    size: u32,
) -> Result<Vec<u8>> {
    let key = api_key
        .map(|s| s.to_string())
        .or_else(|| env::var("STABILITY_API_KEY").ok())
        .ok_or_else(|| {
            IconToolkitError::InvalidConfig(
                "Stability API key missing. Pass --api-key or set STABILITY_API_KEY environment variable."
                    .to_string(),
            )
        })?;

    let dim = (size / 64 * 64).clamp(512, 1024);

    let body = json!({
        "text_prompts": [{"text": prompt, "weight": 1.0}],
        "cfg_scale": 7,
        "height": dim,
        "width": dim,
        "samples": 1,
        "steps": 30
    });

    let res = client
        .post("https://api.stability.ai/v1/generation/stable-diffusion-v1-6/text-to-image")
        .header("Authorization", format!("Bearer {}", key))
        .header("Accept", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| IconToolkitError::AiService {
            provider: "Stability AI".to_string(),
            message: e.to_string(),
        })?;

    if !res.status().is_success() {
        let err_text = res.text().await.unwrap_or_default();
        return Err(IconToolkitError::AiService {
            provider: "Stability AI".to_string(),
            message: format!("API returned HTTP error: {}", err_text),
        });
    }

    let json_res: Value = res.json().await.map_err(|e| IconToolkitError::AiService {
        provider: "Stability AI".to_string(),
        message: format!("Failed to parse JSON response: {}", e),
    })?;

    let b64 = json_res["artifacts"][0]["base64"]
        .as_str()
        .ok_or_else(|| IconToolkitError::AiService {
            provider: "Stability AI".to_string(),
            message: "Response did not contain artifact image payload".to_string(),
        })?;

    base64::engine::general_purpose::STANDARD
        .decode(b64)
        .map_err(|e| IconToolkitError::AiService {
            provider: "Stability AI".to_string(),
            message: format!("Failed to decode base64 image data: {}", e),
        })
}

/// Fetch AI generated image from Gemini / Imagen 3 API.
async fn fetch_gemini_image(
    client: &Client,
    prompt: &str,
    api_key: Option<&str>,
    model: &str,
) -> Result<Vec<u8>> {
    let key = api_key
        .map(|s| s.to_string())
        .or_else(|| env::var("GEMINI_API_KEY").ok())
        .ok_or_else(|| {
            IconToolkitError::InvalidConfig(
                "Gemini API key missing. Pass --api-key or set GEMINI_API_KEY environment variable."
                    .to_string(),
            )
        })?;

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:predict?key={}",
        model, key
    );

    let body = json!({
        "instances": [{"prompt": prompt}],
        "parameters": {
            "sampleCount": 1,
            "aspectRatio": "1:1",
            "outputOptions": {"mimeType": "image/png"}
        }
    });

    let res = client
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| IconToolkitError::AiService {
            provider: "Gemini".to_string(),
            message: e.to_string(),
        })?;

    if !res.status().is_success() {
        let err_text = res.text().await.unwrap_or_default();
        return Err(IconToolkitError::AiService {
            provider: "Gemini".to_string(),
            message: format!("API returned HTTP error: {}", err_text),
        });
    }

    let json_res: Value = res.json().await.map_err(|e| IconToolkitError::AiService {
        provider: "Gemini".to_string(),
        message: format!("Failed to parse JSON response: {}", e),
    })?;

    let b64 = json_res["predictions"][0]["bytesBase64Encoded"]
        .as_str()
        .ok_or_else(|| IconToolkitError::AiService {
            provider: "Gemini".to_string(),
            message: "Response did not contain image bytes payload".to_string(),
        })?;

    base64::engine::general_purpose::STANDARD
        .decode(b64)
        .map_err(|e| IconToolkitError::AiService {
            provider: "Gemini".to_string(),
            message: format!("Failed to decode base64 image data: {}", e),
        })
}

/// Fetch AI generated image from custom OpenAI-compatible endpoint.
async fn fetch_generic_openai_image(
    client: &Client,
    endpoint: &str,
    prompt: &str,
    api_key: Option<&str>,
    model: &str,
    size: u32,
) -> Result<Vec<u8>> {
    let key = api_key
        .map(|s| s.to_string())
        .or_else(|| env::var("OPENAI_API_KEY").ok())
        .unwrap_or_default();

    let size_str = format!("{}x{}", size, size);

    let body = json!({
        "model": model,
        "prompt": prompt,
        "n": 1,
        "size": size_str,
        "response_format": "b64_json"
    });

    let mut req = client.post(endpoint).json(&body);

    if !key.is_empty() {
        req = req.header("Authorization", format!("Bearer {}", key));
    }

    let res = req.send().await.map_err(|e| IconToolkitError::AiService {
        provider: "Generic OpenAI".to_string(),
        message: e.to_string(),
    })?;

    if !res.status().is_success() {
        let err_text = res.text().await.unwrap_or_default();
        return Err(IconToolkitError::AiService {
            provider: "Generic OpenAI".to_string(),
            message: format!("API returned HTTP error: {}", err_text),
        });
    }

    let json_res: Value = res.json().await.map_err(|e| IconToolkitError::AiService {
        provider: "Generic OpenAI".to_string(),
        message: format!("Failed to parse response JSON: {}", e),
    })?;

    if let Some(b64) = json_res["data"][0]["b64_json"].as_str() {
        base64::engine::general_purpose::STANDARD
            .decode(b64)
            .map_err(|e| IconToolkitError::AiService {
                provider: "Generic OpenAI".to_string(),
                message: format!("Failed to decode base64 image data: {}", e),
            })
    } else if let Some(url) = json_res["data"][0]["url"].as_str() {
        // Download image from URL
        let img_res =
            client
                .get(url)
                .send()
                .await
                .map_err(|e| IconToolkitError::AiService {
                    provider: "Generic OpenAI".to_string(),
                    message: format!("Failed to fetch image from URL: {}", e),
                })?;
        let bytes =
            img_res
                .bytes()
                .await
                .map_err(|e| IconToolkitError::AiService {
                    provider: "Generic OpenAI".to_string(),
                    message: format!("Failed to read image bytes: {}", e),
                })?;
        Ok(bytes.to_vec())
    } else {
        Err(IconToolkitError::AiService {
            provider: "Generic OpenAI".to_string(),
            message: "Response did not contain b64_json or image URL".to_string(),
        })
    }
}
