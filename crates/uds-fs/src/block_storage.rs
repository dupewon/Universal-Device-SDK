pub struct BlockStorage;

impl BlockStorage {
    pub fn new() -> Self {
        Self
    }

    pub fn read_block(&self, _block: u32, _buf: &mut [u8]) -> Result<(), String> {
        Ok(())
    }
    pub fn write_block(&self, _block: u32, _data: &[u8]) -> Result<(), String> {
        Ok(())
    }
}
