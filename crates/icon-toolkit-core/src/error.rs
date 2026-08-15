//! Error types for the icon-toolkit core library.
//!
//! Defines `IconToolkitError` which encapsulates all failures during image
//! decoding, resizing, encoding, I/O, format conversion, and AI HTTP requests.

use thiserror::Error;

/// Central error enum for `icon-toolkit-core`.
#[derive(Debug, Error)]
pub enum IconToolkitError {
    /// Error encountered during image loading or decoding.
    #[error("Failed to decode image from path '{path}': {source}")]
    ImageDecode {
        path: String,
        #[source]
        source: image::ImageError,
    },

    /// Error encountered during image encoding.
    #[error("Failed to encode image to format '{format}': {message}")]
    ImageEncode { format: String, message: String },

    /// Error during ICO multi-resolution file construction.
    #[error("ICO encoding error: {0}")]
    IcoEncoding(String),

    /// Error during SVG parsing or rendering.
    #[error("SVG processing error: {0}")]
    SvgProcessing(String),

    /// Standard I/O error.
    #[error("File system I/O error on '{path}': {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },

    /// AI provider HTTP request or response parsing error.
    #[error("AI Service Error ({provider}): {message}")]
    AiService { provider: String, message: String },

    /// Invalid configuration options or input values.
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
}

/// Convenience result alias for `icon-toolkit-core`.
pub type Result<T> = std::result::Result<T, IconToolkitError>;
