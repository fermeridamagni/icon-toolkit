//! Integration tests for icon-toolkit-core using demo-icon.png.

use icon_toolkit_core::{
    convert_image, generate_icons,
    types::{ConvertOptions, IconGeneratorOptions, IconTarget, ImageFormat, Mode},
};
use std::path::PathBuf;

fn get_demo_icon_path() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .join("../..")
        .join("demo-icon.png")
        .canonicalize()
        .expect("demo-icon.png should exist in repo root")
}

#[test]
fn test_generate_web_icons() {
    let demo_path = get_demo_icon_path();
    let out_dir = tempfile::tempdir().unwrap().keep();

    let opts = IconGeneratorOptions {
        input_path: demo_path,
        dark_input_path: None,
        targets: vec![IconTarget::Web],
        mode: Mode::Light,
        output_dir: out_dir.clone(),
        padding_percent: 5,
        background_color: Some("#ffffff".to_string()),
        border_radius_percent: 10,
    };

    let res = generate_icons(&opts).expect("Web icon generation should succeed");
    assert!(!res.created_files.is_empty());

    let web_dir = out_dir.join("web");
    assert!(web_dir.join("favicon.ico").exists());
    assert!(web_dir.join("favicon-16x16.png").exists());
    assert!(web_dir.join("favicon-32x32.png").exists());
    assert!(web_dir.join("apple-touch-icon.png").exists());
    assert!(web_dir.join("android-chrome-192x192.png").exists());
    assert!(web_dir.join("android-chrome-512x512.png").exists());
    assert!(web_dir.join("site.webmanifest").exists());
}

#[test]
fn test_generate_mobile_icons() {
    let demo_path = get_demo_icon_path();
    let out_dir = tempfile::tempdir().unwrap().keep();

    let opts = IconGeneratorOptions {
        input_path: demo_path,
        dark_input_path: None,
        targets: vec![IconTarget::Mobile],
        mode: Mode::Light,
        output_dir: out_dir.clone(),
        padding_percent: 0,
        background_color: None,
        border_radius_percent: 0,
    };

    let res = generate_icons(&opts).expect("Mobile icon generation should succeed");
    assert!(!res.created_files.is_empty());

    let ios_dir = out_dir
        .join("mobile")
        .join("ios")
        .join("AppIcon.appiconset");
    assert!(ios_dir.join("Contents.json").exists());
    assert!(ios_dir.join("Icon-1024.png").exists());

    let android_res = out_dir.join("mobile").join("android").join("res");
    assert!(android_res
        .join("mipmap-xxxhdpi")
        .join("ic_launcher.png")
        .exists());
    assert!(android_res
        .join("mipmap-xxxhdpi")
        .join("ic_launcher_round.png")
        .exists());
}

#[test]
fn test_generate_desktop_icons() {
    let demo_path = get_demo_icon_path();
    let out_dir = tempfile::tempdir().unwrap().keep();

    let opts = IconGeneratorOptions {
        input_path: demo_path,
        dark_input_path: None,
        targets: vec![IconTarget::Desktop],
        mode: Mode::Light,
        output_dir: out_dir.clone(),
        padding_percent: 0,
        background_color: None,
        border_radius_percent: 0,
    };

    let res = generate_icons(&opts).expect("Desktop icon generation should succeed");
    assert!(!res.created_files.is_empty());

    let macos_dir = out_dir
        .join("desktop")
        .join("macos")
        .join("AppIcon.iconset");
    assert!(macos_dir.join("icon_512x512@2x.png").exists());

    let win_ico = out_dir.join("desktop").join("windows").join("icon.ico");
    assert!(win_ico.exists());

    let linux_dir = out_dir.join("desktop").join("linux").join("hicolor");
    assert!(linux_dir
        .join("512x512")
        .join("apps")
        .join("app-icon.png")
        .exists());
}

#[test]
fn test_generate_both_light_and_dark_modes() {
    let demo_path = get_demo_icon_path();
    let out_dir = tempfile::tempdir().unwrap().keep();

    let opts = IconGeneratorOptions {
        input_path: demo_path.clone(),
        dark_input_path: Some(demo_path),
        targets: vec![IconTarget::Web],
        mode: Mode::Both,
        output_dir: out_dir.clone(),
        padding_percent: 0,
        background_color: None,
        border_radius_percent: 0,
    };

    let res = generate_icons(&opts).expect("Light and Dark generation pass should succeed");
    assert!(!res.created_files.is_empty());

    assert!(out_dir
        .join("light")
        .join("web")
        .join("favicon.ico")
        .exists());
    assert!(out_dir
        .join("dark")
        .join("web")
        .join("favicon.ico")
        .exists());
}

#[test]
fn test_convert_image_formats() {
    let demo_path = get_demo_icon_path();
    let out_dir = tempfile::tempdir().unwrap().keep();

    // Convert PNG to WebP
    let webp_out = out_dir.join("demo.webp");
    let webp_res = convert_image(&ConvertOptions {
        input_path: demo_path.clone(),
        output_path: Some(webp_out.clone()),
        format: ImageFormat::Webp,
        quality: 85,
        width: Some(256),
        height: Some(256),
        background_color: None,
    })
    .expect("PNG to WebP conversion should succeed");
    assert!(webp_out.exists());
    assert!(webp_res.file_size > 0);

    // Convert PNG to SVG wrapper
    let svg_out = out_dir.join("demo.svg");
    let svg_res = convert_image(&ConvertOptions {
        input_path: demo_path.clone(),
        output_path: Some(svg_out.clone()),
        format: ImageFormat::Svg,
        quality: 100,
        width: None,
        height: None,
        background_color: None,
    })
    .expect("PNG to SVG conversion should succeed");
    assert!(svg_out.exists());
    assert!(svg_res.file_size > 0);

    // Convert SVG input to PNG via resvg
    let png_from_svg = out_dir.join("rendered_from_svg.png");
    let render_res = convert_image(&ConvertOptions {
        input_path: svg_out,
        output_path: Some(png_from_svg.clone()),
        format: ImageFormat::Png,
        quality: 100,
        width: Some(128),
        height: Some(128),
        background_color: None,
    })
    .expect("SVG to PNG rendering should succeed");
    assert!(png_from_svg.exists());
    assert!(render_res.file_size > 0);

    // Convert PNG to multi-size ICO
    let ico_out = out_dir.join("demo.ico");
    let ico_res = convert_image(&ConvertOptions {
        input_path: demo_path,
        output_path: Some(ico_out.clone()),
        format: ImageFormat::Ico,
        quality: 100,
        width: None,
        height: None,
        background_color: None,
    })
    .expect("PNG to ICO conversion should succeed");
    assert!(ico_out.exists());
    assert!(ico_res.file_size > 0);
}
