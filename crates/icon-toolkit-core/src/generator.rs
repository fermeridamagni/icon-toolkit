//! Icon Generator module for multi-target icon set creation.
//!
//! Generates optimized icon sets for Web, Mobile (iOS AppIcon.appiconset & Android mipmaps),
//! and Desktop (macOS, Windows, Linux) from a single source image, supporting Dark and Light
//! themes, padding, background colors, squircle/border-radius masking, and manifests.

use crate::converter::{apply_background, encode_ico, render_svg_to_image};
use crate::error::{IconToolkitError, Result};
use crate::types::{GeneratorResult, IconGeneratorOptions, IconTarget, Mode};
use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage};
use serde_json::json;
use std::fs::{self, File};
use std::io::BufWriter;
use std::path::Path;

/// Primary entry point for generating icon sets based on `IconGeneratorOptions`.
pub fn generate_icons(options: &IconGeneratorOptions) -> Result<GeneratorResult> {
    if !options.input_path.exists() {
        return Err(IconToolkitError::Io {
            path: options.input_path.display().to_string(),
            source: std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Source image file does not exist",
            ),
        });
    }

    let mut created_files = Vec::new();

    // Determine execution runs for Light / Dark mode
    match options.mode {
        Mode::Both => {
            // Run for light mode
            let light_out = options.output_dir.join("light");
            let light_files = run_generation_pass(
                &options.input_path,
                &light_out,
                &options.targets,
                options.padding_percent,
                &options.background_color,
                options.border_radius_percent,
                "Light",
            )?;
            created_files.extend(light_files);

            // Run for dark mode
            let dark_source = options
                .dark_input_path
                .as_ref()
                .unwrap_or(&options.input_path);
            let dark_out = options.output_dir.join("dark");
            let dark_files = run_generation_pass(
                dark_source,
                &dark_out,
                &options.targets,
                options.padding_percent,
                &options.background_color,
                options.border_radius_percent,
                "Dark",
            )?;
            created_files.extend(dark_files);
        }
        Mode::Dark => {
            let source = options
                .dark_input_path
                .as_ref()
                .unwrap_or(&options.input_path);
            let files = run_generation_pass(
                source,
                &options.output_dir,
                &options.targets,
                options.padding_percent,
                &options.background_color,
                options.border_radius_percent,
                "Dark",
            )?;
            created_files.extend(files);
        }
        Mode::Light => {
            let files = run_generation_pass(
                &options.input_path,
                &options.output_dir,
                &options.targets,
                options.padding_percent,
                &options.background_color,
                options.border_radius_percent,
                "Light",
            )?;
            created_files.extend(files);
        }
    }

    let summary = format!(
        "Successfully generated {} icon file(s) across target(s): {:?}",
        created_files.len(),
        options.targets
    );

    Ok(GeneratorResult {
        created_files,
        summary,
    })
}

/// Run a single generation pass for a specific source image and output directory.
fn run_generation_pass(
    source_path: &Path,
    out_dir: &Path,
    targets: &[IconTarget],
    padding_percent: u32,
    background_color: &Option<String>,
    border_radius_percent: u32,
    mode_label: &str,
) -> Result<Vec<String>> {
    let source_img = load_image(source_path)?;
    let mut files = Vec::new();

    for target in targets {
        match target {
            IconTarget::Web => {
                let target_dir = out_dir.join("web");
                let created = generate_web_target(
                    &source_img,
                    &target_dir,
                    padding_percent,
                    background_color,
                    border_radius_percent,
                )?;
                files.extend(created);
            }
            IconTarget::Mobile => {
                let target_dir = out_dir.join("mobile");
                let created = generate_mobile_target(
                    &source_img,
                    &target_dir,
                    padding_percent,
                    background_color,
                    border_radius_percent,
                )?;
                files.extend(created);
            }
            IconTarget::Desktop => {
                let target_dir = out_dir.join("desktop");
                let created = generate_desktop_target(
                    &source_img,
                    &target_dir,
                    padding_percent,
                    background_color,
                    border_radius_percent,
                )?;
                files.extend(created);
            }
        }
    }

    let _ = mode_label;
    Ok(files)
}

/// Load an image file into `DynamicImage` (supporting PNG, WebP, JPG, SVG).
fn load_image(path: &Path) -> Result<DynamicImage> {
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();
    if ext == "svg" {
        render_svg_to_image(path, Some(1024), Some(1024))
    } else {
        image::open(path).map_err(|e| IconToolkitError::ImageDecode {
            path: path.display().to_string(),
            source: e,
        })
    }
}

/// Prepare a single square icon image with padding, optional background, and border radius.
fn process_icon(
    source: &DynamicImage,
    size: u32,
    padding_percent: u32,
    background_color: &Option<String>,
    border_radius_percent: u32,
) -> Result<DynamicImage> {
    let inner_size = if padding_percent > 0 && padding_percent < 50 {
        let pad_px = (size as f32 * padding_percent as f32 / 100.0) as u32;
        size.saturating_sub(pad_px * 2).max(1)
    } else {
        size
    };

    let resized_source = source.resize_exact(
        inner_size,
        inner_size,
        image::imageops::FilterType::Triangle,
    );

    // Create target transparent canvas
    let mut canvas: RgbaImage = ImageBuffer::new(size, size);
    let offset_x = (size - inner_size) / 2;
    let offset_y = (size - inner_size) / 2;

    // Paste resized image onto canvas
    let resized_rgba = resized_source.to_rgba8();
    for (x, y, pixel) in resized_rgba.enumerate_pixels() {
        let cx = x + offset_x;
        let cy = y + offset_y;
        if cx < size && cy < size {
            canvas.put_pixel(cx, cy, *pixel);
        }
    }

    let mut result_img = DynamicImage::ImageRgba8(canvas);

    // Fill background color if specified
    if let Some(bg_hex) = background_color {
        result_img = apply_background(&result_img, bg_hex)?;
    }

    // Apply border radius / squircle rounding mask if requested
    if border_radius_percent > 0 {
        result_img = apply_border_radius(&result_img, border_radius_percent)?;
    }

    Ok(result_img)
}

/// Apply rounded rectangle / pill mask to an image.
fn apply_border_radius(img: &DynamicImage, radius_percent: u32) -> Result<DynamicImage> {
    let rgba = img.to_rgba8();
    let (width, height) = rgba.dimensions();
    let radius = width.min(height) as f32 * (radius_percent.min(50) as f32 / 100.0);

    let mut masked: RgbaImage = ImageBuffer::new(width, height);

    for (x, y, pixel) in rgba.enumerate_pixels() {
        let px = x as f32 + 0.5;
        let py = y as f32 + 0.5;

        // Calculate distance from nearest corner inside bounding box
        let dx = if px < radius {
            radius - px
        } else if px > width as f32 - radius {
            px - (width as f32 - radius)
        } else {
            0.0
        };

        let dy = if py < radius {
            radius - py
        } else if py > height as f32 - radius {
            py - (height as f32 - radius)
        } else {
            0.0
        };

        let dist_sq = dx * dx + dy * dy;
        let r_sq = radius * radius;

        if dist_sq <= r_sq {
            // Inside smooth corner
            let alpha_factor = if dist_sq > (radius - 1.0) * (radius - 1.0) {
                // Anti-aliased edge
                (radius - dist_sq.sqrt()).clamp(0.0, 1.0)
            } else {
                1.0
            };

            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];
            let a = (pixel[3] as f32 * alpha_factor) as u8;
            masked.put_pixel(x, y, Rgba([r, g, b, a]));
        }
    }

    Ok(DynamicImage::ImageRgba8(masked))
}

/// Generate Web target icon set:
/// - favicon.ico (16x16, 32x32, 48x48)
/// - favicon-16x16.png, favicon-32x32.png
/// - apple-touch-icon.png (180x180)
/// - android-chrome-192x192.png, android-chrome-512x512.png
/// - site.webmanifest
fn generate_web_target(
    source: &DynamicImage,
    web_dir: &Path,
    padding: u32,
    bg: &Option<String>,
    border_radius: u32,
) -> Result<Vec<String>> {
    fs::create_dir_all(web_dir).map_err(|e| IconToolkitError::Io {
        path: web_dir.display().to_string(),
        source: e,
    })?;

    let mut created = Vec::new();

    let png_specs = [
        ("favicon-16x16.png", 16),
        ("favicon-32x32.png", 32),
        ("apple-touch-icon.png", 180),
        ("android-chrome-192x192.png", 192),
        ("android-chrome-512x512.png", 512),
    ];

    for (filename, size) in png_specs {
        let icon_img = process_icon(source, size, padding, bg, border_radius)?;
        let file_path = web_dir.join(filename);
        icon_img
            .save_with_format(&file_path, image::ImageFormat::Png)
            .map_err(|e| IconToolkitError::ImageEncode {
                format: "PNG".to_string(),
                message: e.to_string(),
            })?;
        created.push(file_path.display().to_string());
    }

    // Generate multi-size favicon.ico
    let ico_img = process_icon(source, 256, padding, bg, border_radius)?;
    let ico_path = web_dir.join("favicon.ico");
    encode_ico(&ico_img, &ico_path)?;
    created.push(ico_path.display().to_string());

    // Generate site.webmanifest
    let manifest = json!({
        "name": "App Icon",
        "short_name": "App",
        "icons": [
            {
                "src": "/android-chrome-192x192.png",
                "sizes": "192x192",
                "type": "image/png"
            },
            {
                "src": "/android-chrome-512x512.png",
                "sizes": "512x512",
                "type": "image/png"
            }
        ],
        "theme_color": "#ffffff",
        "background_color": "#ffffff",
        "display": "standalone"
    });

    let manifest_path = web_dir.join("site.webmanifest");
    let manifest_file = File::create(&manifest_path).map_err(|e| IconToolkitError::Io {
        path: manifest_path.display().to_string(),
        source: e,
    })?;
    serde_json::to_writer_pretty(BufWriter::new(manifest_file), &manifest).map_err(|e| {
        IconToolkitError::Io {
            path: manifest_path.display().to_string(),
            source: std::io::Error::other(e),
        }
    })?;
    created.push(manifest_path.display().to_string());

    Ok(created)
}

/// Generate Mobile target icon set:
/// - iOS AppIcon.appiconset with Contents.json
/// - Android res/mipmap-* icon launcher files
fn generate_mobile_target(
    source: &DynamicImage,
    mobile_dir: &Path,
    padding: u32,
    bg: &Option<String>,
    border_radius: u32,
) -> Result<Vec<String>> {
    let mut created = Vec::new();

    // 1. iOS AppIcon.appiconset
    let ios_dir = mobile_dir.join("ios").join("AppIcon.appiconset");
    fs::create_dir_all(&ios_dir).map_err(|e| IconToolkitError::Io {
        path: ios_dir.display().to_string(),
        source: e,
    })?;

    let ios_specs = [
        ("Icon-20@2x.png", 40, "20x20", "2x", "iphone"),
        ("Icon-20@3x.png", 60, "20x20", "3x", "iphone"),
        ("Icon-29@2x.png", 58, "29x29", "2x", "iphone"),
        ("Icon-29@3x.png", 87, "29x29", "3x", "iphone"),
        ("Icon-40@2x.png", 80, "40x40", "2x", "iphone"),
        ("Icon-40@3x.png", 120, "40x40", "3x", "iphone"),
        ("Icon-60@2x.png", 120, "60x60", "2x", "iphone"),
        ("Icon-60@3x.png", 180, "60x60", "3x", "iphone"),
        ("Icon-76@2x.png", 152, "76x76", "2x", "ipad"),
        ("Icon-83.5@2x.png", 167, "83.5x83.5", "2x", "ipad"),
        ("Icon-1024.png", 1024, "1024x1024", "1x", "ios-marketing"),
    ];

    let mut contents_images = Vec::new();

    for (filename, size, size_str, scale, idiom) in ios_specs {
        let icon_img = process_icon(source, size, padding, bg, border_radius)?;
        let file_path = ios_dir.join(filename);
        icon_img
            .save_with_format(&file_path, image::ImageFormat::Png)
            .map_err(|e| IconToolkitError::ImageEncode {
                format: "PNG".to_string(),
                message: e.to_string(),
            })?;
        created.push(file_path.display().to_string());

        contents_images.push(json!({
            "size": size_str,
            "idiom": idiom,
            "filename": filename,
            "scale": scale
        }));
    }

    let contents_json = json!({
        "images": contents_images,
        "info": {
            "version": 1,
            "author": "icon-toolkit"
        }
    });

    let contents_path = ios_dir.join("Contents.json");
    let c_file = File::create(&contents_path).map_err(|e| IconToolkitError::Io {
        path: contents_path.display().to_string(),
        source: e,
    })?;
    serde_json::to_writer_pretty(BufWriter::new(c_file), &contents_json).map_err(|e| {
        IconToolkitError::Io {
            path: contents_path.display().to_string(),
            source: std::io::Error::other(e),
        }
    })?;
    created.push(contents_path.display().to_string());

    // 2. Android mipmaps
    let android_dir = mobile_dir.join("android").join("res");
    let android_specs = [
        ("mipmap-mdpi", 48),
        ("mipmap-hdpi", 72),
        ("mipmap-xhdpi", 96),
        ("mipmap-xxhdpi", 144),
        ("mipmap-xxxhdpi", 192),
    ];

    for (folder, size) in android_specs {
        let folder_path = android_dir.join(folder);
        fs::create_dir_all(&folder_path).map_err(|e| IconToolkitError::Io {
            path: folder_path.display().to_string(),
            source: e,
        })?;

        // Standard launcher icon
        let icon_img = process_icon(source, size, padding, bg, border_radius)?;
        let icon_path = folder_path.join("ic_launcher.png");
        icon_img
            .save_with_format(&icon_path, image::ImageFormat::Png)
            .map_err(|e| IconToolkitError::ImageEncode {
                format: "PNG".to_string(),
                message: e.to_string(),
            })?;
        created.push(icon_path.display().to_string());

        // Round launcher icon (circle rounded)
        let round_img = process_icon(source, size, padding, bg, 50)?;
        let round_path = folder_path.join("ic_launcher_round.png");
        round_img
            .save_with_format(&round_path, image::ImageFormat::Png)
            .map_err(|e| IconToolkitError::ImageEncode {
                format: "PNG".to_string(),
                message: e.to_string(),
            })?;
        created.push(round_path.display().to_string());
    }

    Ok(created)
}

/// Generate Desktop target icon set:
/// - macOS AppIcon.iconset (16, 32, 64, 128, 256, 512, 1024)
/// - Windows icon.ico (multi-resolution 16..256)
/// - Linux hicolor icons
fn generate_desktop_target(
    source: &DynamicImage,
    desktop_dir: &Path,
    padding: u32,
    bg: &Option<String>,
    border_radius: u32,
) -> Result<Vec<String>> {
    let mut created = Vec::new();

    // 1. macOS AppIcon.iconset
    let mac_dir = desktop_dir.join("macos").join("AppIcon.iconset");
    fs::create_dir_all(&mac_dir).map_err(|e| IconToolkitError::Io {
        path: mac_dir.display().to_string(),
        source: e,
    })?;

    let mac_specs = [
        ("icon_16x16.png", 16),
        ("icon_16x16@2x.png", 32),
        ("icon_32x32.png", 32),
        ("icon_32x32@2x.png", 64),
        ("icon_128x128.png", 128),
        ("icon_128x128@2x.png", 256),
        ("icon_256x256.png", 256),
        ("icon_256x256@2x.png", 512),
        ("icon_512x512.png", 512),
        ("icon_512x512@2x.png", 1024),
    ];

    for (filename, size) in mac_specs {
        let icon_img = process_icon(source, size, padding, bg, border_radius)?;
        let file_path = mac_dir.join(filename);
        icon_img
            .save_with_format(&file_path, image::ImageFormat::Png)
            .map_err(|e| IconToolkitError::ImageEncode {
                format: "PNG".to_string(),
                message: e.to_string(),
            })?;
        created.push(file_path.display().to_string());
    }

    // 2. Windows icon.ico
    let win_dir = desktop_dir.join("windows");
    fs::create_dir_all(&win_dir).map_err(|e| IconToolkitError::Io {
        path: win_dir.display().to_string(),
        source: e,
    })?;

    let ico_img = process_icon(source, 256, padding, bg, border_radius)?;
    let win_ico_path = win_dir.join("icon.ico");
    encode_ico(&ico_img, &win_ico_path)?;
    created.push(win_ico_path.display().to_string());

    // 3. Linux hicolor
    let linux_dir = desktop_dir.join("linux").join("hicolor");
    let linux_sizes = [16, 32, 48, 64, 128, 256, 512];

    for size in linux_sizes {
        let size_dir = linux_dir.join(format!("{}x{}", size, size)).join("apps");
        fs::create_dir_all(&size_dir).map_err(|e| IconToolkitError::Io {
            path: size_dir.display().to_string(),
            source: e,
        })?;

        let icon_img = process_icon(source, size, padding, bg, border_radius)?;
        let file_path = size_dir.join("app-icon.png");
        icon_img
            .save_with_format(&file_path, image::ImageFormat::Png)
            .map_err(|e| IconToolkitError::ImageEncode {
                format: "PNG".to_string(),
                message: e.to_string(),
            })?;
        created.push(file_path.display().to_string());
    }

    Ok(created)
}
