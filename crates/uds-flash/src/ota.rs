use crate::image::FirmwareImage;
use crate::progress::ProgressReporter;
use std::sync::atomic::{AtomicU8, Ordering};

pub struct OtaUpdater {
    state: AtomicU8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OtaState {
    Idle = 0,
    Transferring = 1,
    Verifying = 2,
    Applying = 3,
    Complete = 4,
    RolledBack = 5,
    Failed = 6,
}

impl OtaUpdater {
    pub fn new() -> Self {
        Self {
            state: AtomicU8::new(OtaState::Idle as u8),
        }
    }

    pub fn state(&self) -> OtaState {
        match self.state.load(Ordering::SeqCst) {
            1 => OtaState::Transferring,
            2 => OtaState::Verifying,
            3 => OtaState::Applying,
            4 => OtaState::Complete,
            5 => OtaState::RolledBack,
            6 => OtaState::Failed,
            _ => OtaState::Idle,
        }
    }

    pub fn start(&self) -> Result<(), String> {
        if self.state() != OtaState::Idle {
            return Err("OTA already in progress".into());
        }
        self.state
            .store(OtaState::Transferring as u8, Ordering::SeqCst);
        Ok(())
    }

    pub fn update(
        &self,
        image: &FirmwareImage,
        reporter: &dyn ProgressReporter,
    ) -> Result<(), String> {
        reporter.on_progress(0, "Starting OTA update...");

        if image.is_empty() {
            self.state.store(OtaState::Failed as u8, Ordering::SeqCst);
            return Err("Empty firmware image".into());
        }

        self.state
            .store(OtaState::Transferring as u8, Ordering::SeqCst);
        reporter.on_progress(
            20,
            &format!("Transferring {} KB...", image.size_kb() as u32),
        );

        self.state
            .store(OtaState::Verifying as u8, Ordering::SeqCst);
        reporter.on_progress(60, "Verifying image integrity...");

        if !image.verify() {
            self.state.store(OtaState::Failed as u8, Ordering::SeqCst);
            return Err("Image verification failed: checksum mismatch".into());
        }

        self.state.store(OtaState::Applying as u8, Ordering::SeqCst);
        reporter.on_progress(80, "Writing to OTA partition...");

        self.state.store(OtaState::Complete as u8, Ordering::SeqCst);
        reporter.on_progress(100, "Update complete. Rebooting...");
        Ok(())
    }

    pub fn rollback(&self, reporter: &dyn ProgressReporter) -> Result<(), String> {
        reporter.on_progress(0, "Rolling back firmware...");
        std::thread::sleep(std::time::Duration::from_millis(200));
        reporter.on_progress(50, "Restoring previous partition...");
        std::thread::sleep(std::time::Duration::from_millis(200));
        reporter.on_progress(100, "Rollback complete. Rebooting...");
        self.state
            .store(OtaState::RolledBack as u8, Ordering::SeqCst);
        Ok(())
    }

    pub fn reset(&self) {
        self.state.store(OtaState::Idle as u8, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::progress::ConsoleProgress;

    #[test]
    fn test_ota_success() {
        let updater = OtaUpdater::new();
        let image = FirmwareImage::new(b"valid firmware data".to_vec());
        assert!(updater.start().is_ok());
        assert!(updater.update(&image, &ConsoleProgress).is_ok());
        assert_eq!(updater.state(), OtaState::Complete);
    }

    #[test]
    fn test_ota_fails_empty() {
        let updater = OtaUpdater::new();
        let image = FirmwareImage::new(vec![]);
        assert!(updater.start().is_ok());
        assert!(updater.update(&image, &ConsoleProgress).is_err());
        assert_eq!(updater.state(), OtaState::Failed);
    }
}
