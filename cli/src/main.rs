//! CLI binary for `icon-toolkit`.
//!
//! Command-line interface for multi-target icon generation, format conversion,
//! and AI-driven icon synthesis.

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use icon_toolkit_core::{
    convert_image, generate_ai_icon, generate_icons,
    types::{
        AiGeneratorOptions, AiProvider, ConvertOptions, IconGeneratorOptions, IconTarget,
        ImageFormat, Mode,
    },
};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "icon-toolkit",
    author = "icon-toolkit contributors",
    version = "0.1.0",
    about = "Single-source icon generator, format converter, and AI icon generator for Web, Mobile, and Desktop.",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate icon sets for different targets (web, mobile, desktop) from a single source.
    #[command(alias = "g")]
    Generate {
        /// Primary input image path (PNG, WebP, SVG, JPG)
        #[arg(short, long)]
        input: PathBuf,

        /// Dark mode input image path (optional secondary source)
        #[arg(long)]
        dark_input: Option<PathBuf>,

        /// Targets to generate: web, mobile, desktop, or all
        #[arg(
            short,
            long,
            value_delimiter = ',',
            default_value = "web,mobile,desktop"
        )]
        target: Vec<String>,

        /// Theme mode selection: light, dark, or both
        #[arg(short, long, default_value = "light")]
        mode: String,

        /// Output directory path
        #[arg(short, long, default_value = "output/icons")]
        output: PathBuf,

        /// Padding percentage inside icon canvas (0 to 45)
        #[arg(short, long, default_value = "0")]
        padding: u32,

        /// Background color hex code (e.g. "#ffffff" or "transparent")
        #[arg(short, long)]
        background: Option<String>,

        /// Border radius percentage (0 to 50 for rounded corners/squircle/circle)
        #[arg(short = 'r', long, default_value = "0")]
        border_radius: u32,
    },

    /// Convert image to different file formats (png, webp, svg, ico, jpg).
    #[command(alias = "c")]
    Convert {
        /// Source image file path
        #[arg(short, long)]
        input: PathBuf,

        /// Destination output path (inferred if omitted)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Output format: png, webp, svg, ico, jpg
        #[arg(short, long, default_value = "png")]
        format: String,

        /// Quality percentage (1 to 100)
        #[arg(short, long, default_value = "90")]
        quality: u8,

        /// Custom target width
        #[arg(short = 'w', long)]
        width: Option<u32>,

        /// Custom target height
        #[arg(short = 'H', long)]
        height: Option<u32>,

        /// Background color hex for transparent image sources
        #[arg(short, long)]
        background: Option<String>,
    },

    /// Generate icons using AI image models (BYOK: Bring Your Own Key).
    #[command(alias = "a")]
    Ai {
        /// Prompt describing the icon concept
        #[arg(short, long)]
        prompt: String,

        /// AI provider: openai, stability, gemini, or generic
        #[arg(long, default_value = "openai")]
        provider: String,

        /// API key (overrides OPENAI_API_KEY / STABILITY_API_KEY / GEMINI_API_KEY env vars)
        #[arg(long)]
        api_key: Option<String>,

        /// Custom API endpoint URL for generic OpenAI-compatible providers
        #[arg(long)]
        endpoint: Option<String>,

        /// AI Model name (e.g. dall-e-3, imagen-3.0-generate-002)
        #[arg(long)]
        model: Option<String>,

        /// Icon image resolution (e.g. 1024, 512)
        #[arg(short, long, default_value = "1024")]
        size: u32,

        /// Quality setting ("standard" or "hd")
        #[arg(short, long, default_value = "standard")]
        quality: String,

        /// Destination path for generated AI icon image
        #[arg(short, long, default_value = "output/ai-icon.png")]
        output: PathBuf,

        /// Automatically trigger target generator on the AI generated image (web, mobile, desktop)
        #[arg(long, value_delimiter = ',')]
        auto_generate: Option<Vec<String>>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            input,
            dark_input,
            target,
            mode,
            output,
            padding,
            background,
            border_radius,
        } => {
            println!("🚀 Generating icon set from '{}'...", input.display());

            let mut target_enums = Vec::new();
            for t in &target {
                if t.eq_ignore_ascii_case("all") {
                    target_enums = vec![IconTarget::Web, IconTarget::Mobile, IconTarget::Desktop];
                    break;
                } else if let Some(parsed) = IconTarget::from_str_val(t) {
                    target_enums.push(parsed);
                }
            }

            if target_enums.is_empty() {
                target_enums = vec![IconTarget::Web, IconTarget::Mobile, IconTarget::Desktop];
            }

            let opts = IconGeneratorOptions {
                input_path: input,
                dark_input_path: dark_input,
                targets: target_enums,
                mode: Mode::from_str_val(&mode),
                output_dir: output,
                padding_percent: padding,
                background_color: background,
                border_radius_percent: border_radius,
            };

            let res = generate_icons(&opts).context("Failed to generate icons")?;
            println!("✅ {}", res.summary);
            println!("📁 Created {} files:", res.created_files.len());
            for f in &res.created_files {
                println!("  • {}", f);
            }
        }
        Commands::Convert {
            input,
            output,
            format,
            quality,
            width,
            height,
            background,
        } => {
            let img_fmt = ImageFormat::from_ext(&format)
                .ok_or_else(|| anyhow::anyhow!("Unsupported output format: {}", format))?;

            println!(
                "🔄 Converting '{}' to {}...",
                input.display(),
                img_fmt.extension().to_uppercase()
            );

            let opts = ConvertOptions {
                input_path: input,
                output_path: output,
                format: img_fmt,
                quality,
                width,
                height,
                background_color: background,
            };

            let res = convert_image(&opts).context("Failed to convert image")?;
            println!(
                "✅ Converted file saved to '{}' ({} bytes)",
                res.output_path, res.file_size
            );
        }
        Commands::Ai {
            prompt,
            provider,
            api_key,
            endpoint,
            model,
            size,
            quality,
            output,
            auto_generate,
        } => {
            println!("✨ Synthesizing icon with AI prompt: \"{}\"...", prompt);

            let provider_enum = AiProvider::from_str_val(&provider);
            let targets = auto_generate.map(|t_list| {
                t_list
                    .iter()
                    .filter_map(|t| IconTarget::from_str_val(t))
                    .collect()
            });

            let opts = AiGeneratorOptions {
                prompt,
                provider: provider_enum,
                api_key,
                endpoint,
                model,
                size,
                quality,
                output_path: output,
                auto_generate_targets: targets,
            };

            let res = generate_ai_icon(&opts)
                .await
                .context("AI Icon generation failed")?;
            println!("✅ AI Icon saved to '{}'", res.image_path);

            if let Some(target_res) = res.targets_result {
                println!("✅ Auto-generated target icons: {}", target_res.summary);
            }
        }
    }

    Ok(())
}
