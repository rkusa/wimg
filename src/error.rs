use std::any::Any;

#[derive(Debug)]
pub enum Error {
    Resize(resize::Error),
    Jpeg(Box<dyn Any>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Resize(_) => f.write_str("failed to resize image"),
            Error::Jpeg(err) => {
                if let Some(s) = err.downcast_ref::<String>() {
                    write!(f, "failed to process JPEG image: {}", s)
                } else {
                    f.write_str("failed to process JPEG image")
                }
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Resize(err) => Some(err),
            Error::Jpeg(_) => None,
        }
    }
}

impl From<resize::Error> for Error {
    fn from(err: resize::Error) -> Self {
        Self::Resize(err)
    }
}
