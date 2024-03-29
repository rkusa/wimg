use std::collections::BTreeMap;
use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::{Arc, Mutex, RwLock};
use std::time::Instant;
use std::{env, error, fmt, fs, process};

use clap::Parser;
use parallel::*;

#[derive(Debug, Parser)]
#[clap(about, version, author)]
struct Args {
    /// Images that should be transformed
    images: Vec<PathBuf>,

    #[clap(long, short)]
    out_dir: PathBuf,

    #[clap(long, short)]
    base_dir: Option<PathBuf>,

    /// The width the images should be resized to.
    #[clap(long, short)]
    width: u32,

    /// The height the images should be resized to.
    #[clap(long, short)]
    height: u32,

    #[clap(long, short = 'd', default_value = "1")]
    pixel_density: Vec<u8>,

    /// Name of the variant.
    #[clap(long, short = 'n')]
    variant: Option<String>,

    /// Path to the JSON manifest (requires --variant).
    #[clap(long)]
    manifest: Option<PathBuf>,

    #[clap(long, short)]
    format: Vec<OutputFormat>,

    /// Maintain the original image aspect.
    #[clap(long)]
    maintain_aspect: bool,

    #[clap(flatten)]
    jpeg: JpegOptions,

    #[clap(flatten)]
    webp: WebpOptions,

    #[clap(flatten)]
    avif: AvifOptions,
}

#[derive(Debug, clap::Args)]
pub struct JpegOptions {
    /// 0-100 scale
    #[clap(name = "jpeg-quality", long, default_value = "80")]
    pub quality: u16,
}

#[derive(Debug, clap::Args)]
pub struct WebpOptions {
    /// 0-100 scale
    #[clap(name = "webp-quality", long, default_value = "80")]
    pub quality: u16,
}

#[derive(Debug, clap::Args)]
pub struct AvifOptions {
    /// 0-100 scale
    #[clap(name = "avif-quality", long, default_value = "60")]
    pub quality: u16,
    /// rav1e preset 1 (slow) 10 (fast but crappy)
    #[clap(name = "avif-speed", long, default_value = "5")]
    pub speed: u8,
}

#[derive(Debug, Clone)]
enum OutputFormat {
    Avif,
    Jpeg,
    Png,
    Webp,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Variant {
    width: u32,
    height: u32,
    formats: BTreeMap<String, BTreeMap<String, String>>,
}

pub type Manifest = BTreeMap<String, BTreeMap<String, Variant>>;

fn main() {
    let args = Args::parse();
    pretty_env_logger::formatted_builder()
        .filter(Some("wimg_cli"), log::LevelFilter::Debug)
        .init();
    let start = Instant::now();

    if args.format.is_empty() {
        log::error!("no output format specified");
        process::exit(1);
    }

    let current_dir = match env::current_dir() {
        Ok(current_dir) => current_dir,
        Err(err) => {
            log::error!("current working directory is invalid: {}", err);
            process::exit(1);
        }
    };
    let base = match args.base_dir {
        Some(base_dir) if base_dir.is_dir() => {
            if base_dir.is_relative() {
                current_dir.join(base_dir)
            } else {
                base_dir
            }
        }
        Some(_) => {
            log::error!("--base-dir is not a valid directory");
            process::exit(1);
        }
        None => current_dir.clone(),
    };
    log::debug!("Base dir: {}", base.to_string_lossy());

    let manifest = args.manifest.as_ref().map(|path| {
        let variant = match args.variant {
            Some(variant) => variant,
            None => {
                log::error!(
                    "When writing into a manifest (--manifest), the variant name (--variant) \
                            is required",
                );
                process::exit(1);
            }
        };

        if path.is_file() {
            let data = match fs::read(path) {
                Ok(data) => data,
                Err(err) => {
                    log::error!(
                        "failed to read manifest {} ({})",
                        path.to_string_lossy(),
                        err
                    );
                    process::exit(1);
                }
            };
            match serde_json::from_slice::<Manifest>(&data) {
                Ok(manifest) => (Mutex::new(manifest), variant),
                Err(err) => {
                    log::error!("failed to parse existing manifest as JSON: {}", err);
                    process::exit(1);
                }
            }
        } else {
            (Mutex::new(Manifest::default()), variant)
        }
    });

    let images = args
        .images
        .into_iter()
        .map(|mut path| {
            if path.is_relative() {
                path = current_dir.join(path)
            }

            if !path.is_file() {
                log::error!("{} is not a valid file", path.to_string_lossy());
                process::exit(1);
            }

            if path.is_absolute() && !path.starts_with(&base) {
                log::error!(
                    "{} is outside of the base directory",
                    path.to_string_lossy()
                );
                process::exit(1);
            }

            path
        })
        .collect::<Vec<_>>();

    if images.is_empty() {
        log::error!("No input images provided");
        process::exit(1);
    }

    images.par_iter().for_each(|path| {
        let path_string = path.to_string_lossy();
        log::debug!("Processing {}", path_string);
        let data = match fs::read(path) {
            Ok(data) => data,
            Err(err) => {
                log::error!("failed to read {} ({})", path_string, err);
                process::exit(1);
            }
        };

        let result = match path.extension().and_then(|e| e.to_str()) {
            Some("jpg") => wimg::jpeg::decode(&data),
            Some("png") => wimg::png::decode(&data),
            Some(ext) => {
                log::error!("unsupported image format: {}", ext);
                process::exit(1);
            }
            None => {
                log::error!(
                    "{} must have an extension to guess the image format from",
                    path_string
                );
                process::exit(1);
            }
        };
        let image = match result {
            Ok(image) => Arc::new(RwLock::new(image)),
            Err(err) => {
                log::error!("failed to decode {}: {}", path_string, err);
                process::exit(1);
            }
        };

        args.pixel_density.par_iter().for_each(|pd| {
            let pd = *pd;
            if pd == 0 {
                return;
            }

            log::debug!("Resizing {} (PD: {})", path_string, pd);
            let image = image.read().unwrap();
            let image = match wimg::resize::resize(
                &image,
                args.width * pd as u32,
                args.height * pd as u32,
                args.maintain_aspect,
            ) {
                Ok(image) => image,
                Err(err) => {
                    log::error!("failed to resize {}: {}", path_string, err);
                    process::exit(1);
                }
            };

            let relative_path = path.strip_prefix(&base).unwrap();
            let name = relative_path.to_string_lossy().to_string();
            let out_file = args.out_dir.join(relative_path);

            args.format.par_iter().for_each(|format| {
                let seed = wimg::resize::seed()
                    + match format {
                        OutputFormat::Avif => wimg::avif::seed(),
                        OutputFormat::Jpeg => wimg::jpeg::seed(),
                        OutputFormat::Png => wimg::png::seed(),
                        OutputFormat::Webp => wimg::webp::seed(),
                    };
                let mut hash = wimg::hash::hash(&data, seed);
                if let Some((_, variant)) = &manifest {
                    hash += wimg::hash::hash(variant.as_bytes(), seed);
                }
                let hash = hex::encode(hash.to_be_bytes());

                let file_stem = out_file
                    .file_stem()
                    .and_then(|n| n.to_str())
                    .unwrap_or_default();
                let out_file = out_file
                    .with_file_name(if pd > 1 {
                        format!("{file_stem}-{hash}@{pd}x")
                    } else {
                        format!("{file_stem}-{hash}")
                    })
                    .with_extension(format.ext());
                log::debug!("Writing to {}", out_file.to_string_lossy());

                if let Some(parent) = out_file.parent() {
                    if let Err(err) = fs::create_dir_all(parent) {
                        log::error!(
                            "failed to create directory {}: {}",
                            parent.to_string_lossy(),
                            err
                        );
                        process::exit(1);
                    }
                }

                let result = match format {
                    OutputFormat::Avif => wimg::avif::encode(&image, &(&args.avif).into()),
                    OutputFormat::Jpeg => wimg::jpeg::encode(&image, &(&args.jpeg).into()),
                    OutputFormat::Png => wimg::png::encode(&image),
                    OutputFormat::Webp => wimg::webp::encode(&image, &(&args.webp).into()),
                };
                let image = match result {
                    Ok(image) => image,
                    Err(err) => {
                        log::error!("failed to encode {} as {}: {}", path_string, format, err);
                        process::exit(1);
                    }
                };

                if let Err(err) = fs::write(&out_file, &image) {
                    log::error!("failed to write {}: {}", out_file.to_string_lossy(), err);
                    process::exit(1);
                }

                if let Some((manifest, variant)) = &manifest {
                    let mut manifest = manifest.lock().unwrap();
                    let variants = manifest.entry(name.to_string()).or_default();
                    let variant = variants.entry(variant.clone()).or_insert_with(|| Variant {
                        width: image.width() / pd as u32,
                        height: image.height() / pd as u32,
                        formats: Default::default(),
                    });
                    let pixel_densities = variant
                        .formats
                        .entry(format.mime_type().to_string())
                        .or_default();
                    pixel_densities.insert(
                        format!("{pd}x"),
                        out_file
                            .strip_prefix(&args.out_dir)
                            .unwrap()
                            .to_string_lossy()
                            .to_string(),
                    );
                }
            });
        });
    });

    if let Some((manifest, _)) = manifest {
        let file = match File::create(args.manifest.unwrap()) {
            Ok(file) => file,
            Err(err) => {
                log::error!("failed to write manifest: {}", err);
                process::exit(1);
            }
        };

        if let Err(err) = serde_json::to_writer_pretty(file, &manifest) {
            log::error!("failed to write manifest: {}", err);
            process::exit(1);
        }
    }

    log::debug!("Took: {:?}", start.elapsed());
}

impl OutputFormat {
    fn ext(&self) -> &'static str {
        match self {
            OutputFormat::Avif => "avif",
            OutputFormat::Jpeg => "jpg",
            OutputFormat::Png => "png",
            OutputFormat::Webp => "webp",
        }
    }

    fn mime_type(&self) -> &'static str {
        match self {
            OutputFormat::Avif => "image/avif",
            OutputFormat::Jpeg => "image/jpeg",
            OutputFormat::Png => "image/png",
            OutputFormat::Webp => "image/webp",
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.ext())
    }
}

impl FromStr for OutputFormat {
    type Err = ParseOutputFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_lowercase().as_str() {
            "avif" => OutputFormat::Avif,
            "jpg" | "jpeg" => OutputFormat::Jpeg,
            "png" => OutputFormat::Png,
            "webp" => OutputFormat::Webp,
            _ => return Err(ParseOutputFormatError),
        })
    }
}

#[derive(Debug)]
struct ParseOutputFormatError;

impl fmt::Display for ParseOutputFormatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid output format")
    }
}

impl error::Error for ParseOutputFormatError {}

impl<'a> From<&'a JpegOptions> for wimg::jpeg::EncodeOptions {
    fn from(opts: &'a JpegOptions) -> Self {
        Self {
            quality: opts.quality,
        }
    }
}

impl<'a> From<&'a WebpOptions> for wimg::webp::EncodeOptions {
    fn from(opts: &'a WebpOptions) -> Self {
        Self {
            quality: opts.quality,
        }
    }
}

impl<'a> From<&'a AvifOptions> for wimg::avif::EncodeOptions {
    fn from(opts: &'a AvifOptions) -> Self {
        Self {
            quality: opts.quality,
            speed: opts.speed,
        }
    }
}

#[cfg(feature = "parallel")]
mod parallel {
    pub use rayon::prelude::*;
}

#[cfg(not(feature = "parallel"))]
mod parallel {
    pub trait IntoRefIterator<'data> {
        type Iter: IntoIterator<Item = Self::Item>;
        type Item: Send + 'data;
        fn par_iter(&'data self) -> Self::Iter;
    }

    impl<'data, I: 'data + ?Sized> IntoRefIterator<'data> for I
    where
        &'data I: IntoIterator,
        <&'data I as IntoIterator>::Item: Send,
    {
        type Iter = <&'data I as IntoIterator>::IntoIter;
        type Item = <&'data I as IntoIterator>::Item;

        fn par_iter(&'data self) -> Self::Iter {
            self.into_iter()
        }
    }
}
