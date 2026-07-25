use sha2::{Sha256, Digest};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct FirmwareImage {
    pub data: Vec<u8>,
    pub checksum: [u8; 32],
    pub path: Option<String>,
}

impl FirmwareImage {
    pub fn new(data: Vec<u8>) -> Self {
        let hash = Sha256::digest(&data);
        Self { data, checksum: hash.into(), path: None }
    }

    pub fn from_file(path: &str) -> Result<Self, String> {
        let p = Path::new(path);
        if !p.exists() {
            return Err(format!("file not found: {}", path));
        }
        let data = std::fs::read(p).map_err(|e| e.to_string())?;
        let mut img = Self::new(data);
        img.path = Some(path.to_string());
        Ok(img)
    }

    pub fn verify(&self) -> bool {
        let hash = Sha256::digest(&self.data);
        hash.as_slice() == self.checksum
    }

    pub fn size_kb(&self) -> f64 {
        self.data.len() as f64 / 1024.0
    }

    pub fn size_mb(&self) -> f64 {
        self.data.len() as f64 / (1024.0 * 1024.0)
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_firmware_image_creation() {
        let data = b"test firmware binary data".to_vec();
        let img = FirmwareImage::new(data.clone());
        assert_eq!(img.data, data);
        assert!(img.verify());
    }

    #[test]
    fn test_firmware_verification_fails() {
        let mut img = FirmwareImage::new(b"original data".to_vec());
        img.data = b"tampered data".to_vec().into();
        assert!(!img.verify());
    }
}
