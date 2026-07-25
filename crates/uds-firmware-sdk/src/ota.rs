use sha2::{Sha256, Digest};

const MAX_FIRMWARE_SIZE: usize = 1024 * 1024;
const CHUNK_SIZE: usize = 256;

enum OtaState {
    Idle,
    Receiving { offset: u32, expected_size: u32, hash: Sha256 },
    Complete,
}

pub struct OtaClient {
    state: OtaState,
    buffer: [u8; MAX_FIRMWARE_SIZE],
    received: u32,
}

impl OtaClient {
    pub fn new() -> Self {
        Self {
            state: OtaState::Idle,
            buffer: [0u8; MAX_FIRMWARE_SIZE],
            received: 0,
        }
    }

    pub fn begin_update(&mut self, expected_size: u32) {
        self.state = OtaState::Receiving {
            offset: 0,
            expected_size,
            hash: Sha256::new(),
        };
        self.received = 0;
    }

    pub fn receive_chunk(&mut self, offset: u32, data: &[u8]) -> bool {
        match &mut self.state {
            OtaState::Receiving { offset: expected_offset, expected_size, hash } => {
                if offset != *expected_offset {
                    return false;
                }
                if data.len() > CHUNK_SIZE || (offset as usize + data.len()) > MAX_FIRMWARE_SIZE {
                    return false;
                }
                let end = offset as usize + data.len();
                if end as u32 > *expected_size {
                    return false;
                }
                self.buffer[offset as usize..end].copy_from_slice(data);
                hash.update(data);
                *expected_offset += data.len() as u32;
                self.received += data.len() as u32;
                if self.received >= *expected_size {
                    let _computed = hash.clone().finalize();
                    self.state = OtaState::Complete;
                }
                true
            }
            _ => false,
        }
    }

    pub fn progress(&self) -> f32 {
        match &self.state {
            OtaState::Receiving { expected_size, .. } if *expected_size > 0 => {
                self.received as f32 / *expected_size as f32
            }
            OtaState::Complete => 1.0,
            _ => 0.0,
        }
    }

    pub fn verify(&self) -> bool {
        matches!(self.state, OtaState::Complete)
    }

    pub fn apply(&mut self) -> bool {
        if !matches!(self.state, OtaState::Complete) {
            return false;
        }
        self.state = OtaState::Idle;
        self.received = 0;
        true
    }
}
