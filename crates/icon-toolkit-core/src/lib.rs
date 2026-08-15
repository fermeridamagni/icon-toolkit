//! # icon-toolkit-core
//!
//! Pure Rust core engine for:
//! - Multi-target icon set generation (Web, Mobile iOS/Android, Desktop macOS/Windows/Linux)
//! - Dark & Light mode icon variants, padding, background colors, and border-radius masks
//! - High quality image format conversions (`PNG`, `WebP`, `SVG`, `ICO`, `JPG`)
//! - AI Icon Generation with Bring Your Own Key (BYOK) for OpenAI, Stability AI, Gemini, etc.

pub mod ai;
pub mod converter;
pub mod error;
pub mod generator;
pub mod types;

pub use ai::generate_ai_icon;
pub use converter::convert_image;
pub use error::{IconToolkitError, Result};
pub use generator::generate_icons;
pub use types::*;
