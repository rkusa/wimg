use crate::ImageFormat;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to resize image")]
    Resize(#[from] resize::Error),
    #[error("failed to process JPEG image: {0}")]
    Jpeg(#[from] jpeg::JpegError),
    #[error("failed to process PNG image")]
    Png(#[from] crate::png::PngError),
    #[error("failed to process AVIF image")]
    Avif(#[from] ravif::Error),
    #[error("failed to process WEBP image: {0}")]
    Webp(&'static str),
    #[error("crop out of bounds")]
    CropOutOfBounds,
    #[error("received null pointer")]
    NullPtr,
    #[error("cannot {process} {format}")]
    Process {
        process: &'static str,
        format: ImageFormat,
    },
}
