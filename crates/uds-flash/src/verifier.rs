use crate::image::FirmwareImage;

pub struct ImageVerifier;

impl ImageVerifier {
    pub fn verify(image: &FirmwareImage) -> bool {
        image.verify()
    }
}
