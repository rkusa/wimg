use libwebp_sys::{
    WebPConfig, WebPConfigPreset, WebPEncode, WebPMemoryWrite, WebPMemoryWriter,
    WebPMemoryWriterClear, WebPMemoryWriterInit, WebPPicture, WebPPictureFree,
    WebPPictureImportRGB, WebPPictureImportRGBA, WebPPictureInit, WebPValidateConfig,
    WEBP_PRESET_PHOTO,
};
use mozjpeg_sys::c_int;

use crate::error::Error;
use crate::{Image, ImageFormat};

pub fn seed() -> u32 {
    0
}

pub fn encode(img: &Image) -> Result<Image, Error> {
    unsafe {
        let mut config: WebPConfig = std::mem::zeroed();
        if WebPConfigPreset(&mut config, WEBP_PRESET_PHOTO, 80.0) == 0 {
            return Err(Error::Webp("failed to initialize config preset"));
        }
        if WebPValidateConfig(&config) == 0 {
            return Err(Error::Webp("invalid config"));
        }

        let mut picture: WebPPicture = std::mem::zeroed();
        if WebPPictureInit(&mut picture) == 0 {
            return Err(Error::Webp("failed to initialize picture"));
        }
        picture.width = img.width as i32;
        picture.height = img.height as i32;

        let mut writer: WebPMemoryWriter = std::mem::zeroed();
        WebPMemoryWriterInit(&mut writer);
        picture.writer = Some(write);
        picture.custom_ptr = (&mut writer) as *mut WebPMemoryWriter as *mut _;

        let ok = match img.format {
            ImageFormat::RGB8 => {
                WebPPictureImportRGB(&mut picture, img.as_ref().as_ptr(), (img.width * 3) as i32)
            }
            ImageFormat::RGBA8 => {
                WebPPictureImportRGBA(&mut picture, img.as_ref().as_ptr(), (img.width * 4) as i32)
            }
            _ => {
                return Err(Error::Process {
                    process: "encode as WebP",
                    format: img.format,
                })
            }
        };
        if ok == 0 {
            return Err(Error::Webp("failed to import pixel data"));
        }

        let ok = WebPEncode(&config, &mut picture);
        WebPPictureFree(&mut picture);
        if ok == 0 {
            WebPMemoryWriterClear(&mut writer);
            return Err(Error::Webp("failed to encode"));
        }

        let data = Vec::from_raw_parts(writer.mem, writer.size, writer.max_size);
        Ok(Image::new(data, ImageFormat::WEBP, img.width, img.height))
    }
}

extern "C" fn write(data: *const u8, data_size: usize, picture: *const WebPPicture) -> c_int {
    unsafe { WebPMemoryWrite(data, data_size, picture) }
}
