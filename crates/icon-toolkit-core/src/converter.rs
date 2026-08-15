//! Image format converter module.
//!
//! Provides `convert_image` to transform images between PNG, WebP, SVG, ICO, and JPG formats.
//! Supports SVG vector canvas generation, high-fidelity SVG rasterization via `resvg`,
//! background color blending, custom quality settings, and resizing.

use crate::error::{IconToolkitError, Result};
use crate::types::{ConvertOptions, ConvertResult, ImageFormat};
use base64::Engine;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};
use std::fs::{self, File};
use std::io::{BufWriter, Cursor};
use std::path::Path;

/// Primary entry point for converting an image based on `ConvertOptions`.
pub fn convert_image(options: &ConvertOptions) -> Result<ConvertResult> {
    if !options.input_path.exists() {
        return Err(IconToolkitError::Io {
            path: options.input_path.display().to_string(),
            source: std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Input image file does not exist",
            ),
        });
    }

    // Determine output file path
    let output_path = match &options.output_path {
        Some(p) => p.clone(),
        None => {
            let mut p = options.input_path.clone();
            p.set_extension(options.format.extension());
            p
        }
    };

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).map_err(|e| IconToolkitError::Io {
            path: parent.display().to_string(),
            source: e,
        })?;
    }

    // Check if input is SVG
    let input_ext = options
        .input_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    let mut dynamic_img = if input_ext == "svg" {
        render_svg_to_image(&options.input_path, options.width, options.height)?
    } else {
        image::open(&options.input_path).map_err(|e| IconToolkitError::ImageDecode {
            path: options.input_path.display().to_string(),
            source: e,
        })?
    };

    // Apply resizing if requested (and wasn't SVG pre-sized)
    if input_ext != "svg" && (options.width.is_some() || options.height.is_some()) {
        let (orig_w, orig_h) = dynamic_img.dimensions();
        let target_w = options.width.unwrap_or(orig_w);
        let target_h = options.height.unwrap_or(orig_h);
        dynamic_img =
            dynamic_img.resize_exact(target_w, target_h, image::imageops::FilterType::Triangle);
    }

    // Apply background color fill if requested
    if let Some(bg_hex) = &options.background_color {
        dynamic_img = apply_background(&dynamic_img, bg_hex)?;
    }

    // Encode to destination format
    match options.format {
        ImageFormat::Png => {
            dynamic_img
                .save_with_format(&output_path, image::ImageFormat::Png)
                .map_err(|e| IconToolkitError::ImageEncode {
                    format: "PNG".to_string(),
                    message: e.to_string(),
                })?;
        }
        ImageFormat::Webp => {
            encode_webp(&dynamic_img, &output_path, options.quality)?;
        }
        ImageFormat::Jpg => {
            let rgb_img = dynamic_img.to_rgb8();
            rgb_img
                .save_with_format(&output_path, image::ImageFormat::Jpeg)
                .map_err(|e| IconToolkitError::ImageEncode {
                    format: "JPEG".to_string(),
                    message: e.to_string(),
                })?;
        }
        ImageFormat::Ico => {
            encode_ico(&dynamic_img, &output_path)?;
        }
        ImageFormat::Svg => {
            encode_svg_wrapper(&dynamic_img, &output_path)?;
        }
    }

    let metadata = fs::metadata(&output_path).map_err(|e| IconToolkitError::Io {
        path: output_path.display().to_string(),
        source: e,
    })?;

    Ok(ConvertResult {
        output_path: output_path.display().to_string(),
        format: options.format.extension().to_string(),
        file_size: metadata.len(),
    })
}

/// Render an SVG file to a DynamicImage using `resvg`.
pub fn render_svg_to_image(
    svg_path: &Path,
    target_width: Option<u32>,
    target_height: Option<u32>,
) -> Result<DynamicImage> {
    let svg_data = fs::read(svg_path).map_err(|e| IconToolkitError::Io {
        path: svg_path.display().to_string(),
        source: e,
    })?;

    let opt = usvg::Options::default();
    let tree = usvg::Tree::from_data(&svg_data, &opt)
        .map_err(|e| IconToolkitError::SvgProcessing(format!("Failed to parse SVG file: {}", e)))?;

    let orig_size = tree.size();
    let (width, height) = match (target_width, target_height) {
        (Some(w), Some(h)) => (w, h),
        (Some(w), None) => {
            let h = (w as f32 * orig_size.height() / orig_size.width()) as u32;
            (w, h.max(1))
        }
        (None, Some(h)) => {
            let w = (h as f32 * orig_size.width() / orig_size.height()) as u32;
            (w.max(1), h)
        }
        (None, None) => (orig_size.width() as u32, orig_size.height() as u32),
    };

    let mut pixmap = tiny_skia::Pixmap::new(width, height).ok_or_else(|| {
        IconToolkitError::SvgProcessing(format!(
            "Failed to allocate pixmap buffer for dimensions {}x{}",
            width, height
        ))
    })?;

    let transform = tiny_skia::Transform::from_scale(
        width as f32 / orig_size.width(),
        height as f32 / orig_size.height(),
    );

    resvg::render(&tree, transform, &mut pixmap.as_mut());

    let rgba_buffer = pixmap.data().to_vec();
    let img_buf = RgbaImage::from_raw(width, height, rgba_buffer).ok_or_else(|| {
        IconToolkitError::SvgProcessing("Failed to construct RGBA image buffer".to_string())
    })?;

    Ok(DynamicImage::ImageRgba8(img_buf))
}

/// Encode RGBA image into an SVG document with embedded lossless PNG data URI canvas.
pub fn encode_svg_wrapper(img: &DynamicImage, output_path: &Path) -> Result<()> {
    let (width, height) = img.dimensions();
    let mut png_bytes = Vec::new();
    img.write_to(&mut Cursor::new(&mut png_bytes), image::ImageFormat::Png)
        .map_err(|e| IconToolkitError::ImageEncode {
            format: "SVG embedded PNG".to_string(),
            message: e.to_string(),
        })?;

    let base64_png = base64::engine::general_purpose::STANDARD.encode(&png_bytes);
    let svg_content = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {w} {h}" width="{w}" height="{h}">
  <image width="{w}" height="{h}" href="data:image/png;base64,{data}"/>
</svg>
"#,
        w = width,
        h = height,
        data = base64_png
    );

    fs::write(output_path, svg_content).map_err(|e| IconToolkitError::Io {
        path: output_path.display().to_string(),
        source: e,
    })?;

    Ok(())
}

/// Encode image to WebP format.
pub fn encode_webp(img: &DynamicImage, output_path: &Path, _quality: u8) -> Result<()> {
    // image crate supports webp save
    img.save_with_format(output_path, image::ImageFormat::WebP)
        .map_err(|e| IconToolkitError::ImageEncode {
            format: "WEBP".to_string(),
            message: e.to_string(),
        })
}

/// Encode image into multi-size Windows ICO file (containing 16x16, 32x32, 48x48, 64x64, 128x128, 256x256).
pub fn encode_ico(img: &DynamicImage, output_path: &Path) -> Result<()> {
    let sizes = [16, 32, 48, 64, 128, 256];
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);

    for size in sizes {
        let resized = img.resize_exact(size, size, image::imageops::FilterType::Triangle);
        let rgba = resized.to_rgba8();
        let ico_image = ico::IconImage::from_rgba_data(size, size, rgba.into_raw());
        icon_dir.add_entry(ico::IconDirEntry::encode(&ico_image).map_err(|e| {
            IconToolkitError::IcoEncoding(format!(
                "Failed to encode {}x{} frame for ICO: {}",
                size, size, e
            ))
        })?);
    }

    let file = File::create(output_path).map_err(|e| IconToolkitError::Io {
        path: output_path.display().to_string(),
        source: e,
    })?;

    icon_dir
        .write(BufWriter::new(file))
        .map_err(|e| IconToolkitError::IcoEncoding(e.to_string()))?;

    Ok(())
}

/// Apply background color fill to an image with transparency.
pub fn apply_background(img: &DynamicImage, hex_color: &str) -> Result<DynamicImage> {
    let color = parse_hex_color(hex_color)?;
    let rgba_img = img.to_rgba8();
    let (width, height) = rgba_img.dimensions();
    let mut out_img: RgbaImage = ImageBuffer::new(width, height);

    for (x, y, pixel) in rgba_img.enumerate_pixels() {
        let alpha = pixel[3] as f32 / 255.0;
        let inv_alpha = 1.0 - alpha;

        let r = (pixel[0] as f32 * alpha + color[0] as f32 * inv_alpha) as u8;
        let g = (pixel[1] as f32 * alpha + color[1] as f32 * inv_alpha) as u8;
        let b = (pixel[2] as f32 * alpha + color[2] as f32 * inv_alpha) as u8;
        let a = (255.0 * (alpha + color[3] as f32 / 255.0 * inv_alpha)) as u8;

        out_img.put_pixel(x, y, Rgba([r, g, b, a]));
    }

    Ok(DynamicImage::ImageRgba8(out_img))
}

/// Helper function to parse hex color code (e.g. "#ffffff", "#00000080", "transparent").
pub fn parse_hex_color(hex: &str) -> Result<[u8; 4]> {
    let clean = hex.trim().to_lowercase();
    if clean == "transparent" {
        return Ok([0, 0, 0, 0]);
    }

    let s = clean.strip_prefix('#').unwrap_or(&clean);
    match s.len() {
        6 => {
            let r = u8::from_str_radix(&s[0..2], 16).map_err(|_| {
                IconToolkitError::InvalidConfig(format!("Invalid hex color: {}", hex))
            })?;
            let g = u8::from_str_radix(&s[2..4], 16).map_err(|_| {
                IconToolkitError::InvalidConfig(format!("Invalid hex color: {}", hex))
            })?;
            let b = u8::from_str_radix(&s[4..6], 16).map_err(|_| {
                IconToolkitError::InvalidConfig(format!("Invalid hex color: {}", hex))
            })?;
            Ok([r, g, b, 255])
        }
        8 => {
            let r = u8::from_str_radix(&s[0..2], 16).map_err(|_| {
                IconToolkitError::InvalidConfig(format!("Invalid hex color: {}", hex))
            })?;
            let g = u8::from_str_radix(&s[2..4], 16).map_err(|_| {
                IconToolkitError::InvalidConfig(format!("Invalid hex color: {}", hex))
            })?;
            let b = u8::from_str_radix(&s[4..6], 16).map_err(|_| {
                IconToolkitError::InvalidConfig(format!("Invalid hex color: {}", hex))
            })?;
            let a = u8::from_str_radix(&s[6..8], 16).map_err(|_| {
                IconToolkitError::InvalidConfig(format!("Invalid hex color: {}", hex))
            })?;
            Ok([r, g, b, a])
        }
        _ => Err(IconToolkitError::InvalidConfig(format!(
            "Invalid hex color format: {}",
            hex
        ))),
    }
}
