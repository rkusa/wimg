use crate::ImageFormat;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to resize image")]
    Resize(#[from] resize::Error),
    #[error("failed to process JPEG image: {0}")]
    Jpeg(Box<String>),
    #[error("failed to process PNG image")]
    Png(#[from] crate::png::PngError),
    #[error("failed to process AVIF image")]
    Avif(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("crop out of bounds")]
    CropOutOfBounds,
    #[error("received null pointer")]
    NullPtr,
    #[error("cannot {process} {format}")]
    Process {
        process: &'static str,
        format: ImageFormat,
    },
    #[error("failed to write to output buffer")]
    Io(#[from] std::io::Error),
}
