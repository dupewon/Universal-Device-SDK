use crate::error::FsError;

pub trait FileSystem {
    fn list_dir(&self, path: &str) -> Result<Vec<String>, FsError>;
    fn read_file(&self, path: &str) -> Result<Vec<u8>, FsError>;
    fn write_file(&self, path: &str, data: &[u8]) -> Result<(), FsError>;
    fn remove(&self, path: &str) -> Result<(), FsError>;
    fn create_dir(&self, path: &str) -> Result<(), FsError>;
}

pub struct FsOperations;

impl Default for FsOperations {
    fn default() -> Self {
        Self::new()
    }
}

impl FsOperations {
    pub fn new() -> Self {
        Self
    }
}

impl FileSystem for FsOperations {
    fn list_dir(&self, _path: &str) -> Result<Vec<String>, FsError> {
        Ok(vec![])
    }
    fn read_file(&self, _path: &str) -> Result<Vec<u8>, FsError> {
        Ok(vec![])
    }
    fn write_file(&self, _path: &str, _data: &[u8]) -> Result<(), FsError> {
        Ok(())
    }
    fn remove(&self, _path: &str) -> Result<(), FsError> {
        Ok(())
    }
    fn create_dir(&self, _path: &str) -> Result<(), FsError> {
        Ok(())
    }
}
