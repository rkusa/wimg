use std::any::Any;

use crate::ImageFormat;

#[derive(Debug)]
pub enum Error {
    Resize(resize::Error),
    Jpeg(Box<dyn Any>),
    CropOutOfBounds,
    NullPtr,
    Process {
        process: &'static str,
        format: ImageFormat,
    },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Resize(_) => f.write_str("failed to resize image"),
            Self::Jpeg(err) => {
                if let Some(s) = err.downcast_ref::<String>() {
                    write!(f, "failed to process JPEG image: {}", s)
                } else {
                    f.write_str("failed to process JPEG image")
                }
            }
            Self::CropOutOfBounds => f.write_str("crop out of bounds"),
            Self::NullPtr => f.write_str("received null pointer"),
            Self::Process { process, format } => write!(f, "cannot {} {}", process, format),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Resize(err) => Some(err),
            Self::Jpeg(_) => None,
            Self::CropOutOfBounds => None,
            Self::NullPtr => None,
            Self::Process { .. } => None,
        }
    }
}

impl From<resize::Error> for Error {
    fn from(err: resize::Error) -> Self {
        Self::Resize(err)
    }
}
