#[derive(Debug, Clone)]
pub struct Partition {
    pub name: String,
    pub offset: u32,
    pub size: u32,
    pub partition_type: PartitionType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitionType {
    Bootloader,
    App,
    OtaData,
    OtaOld,
    OtaNew,
    Spiffs,
    Nvs,
    Unknown,
}

impl PartitionType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "bootloader" => Self::Bootloader,
            "app" => Self::App,
            "otadata" => Self::OtaData,
            "ota_0" | "otaold" => Self::OtaOld,
            "ota_1" | "otanew" => Self::OtaNew,
            "spiffs" => Self::Spiffs,
            "nvs" => Self::Nvs,
            _ => Self::Unknown,
        }
    }
}

pub struct PartitionManager {
    partitions: Vec<Partition>,
}

impl PartitionManager {
    pub fn new() -> Self {
        Self {
            partitions: vec![
                Partition {
                    name: "bootloader".into(),
                    offset: 0x1000,
                    size: 0x8000,
                    partition_type: PartitionType::Bootloader,
                },
                Partition {
                    name: "nvs".into(),
                    offset: 0x9000,
                    size: 0x6000,
                    partition_type: PartitionType::Nvs,
                },
                Partition {
                    name: "otadata".into(),
                    offset: 0xF000,
                    size: 0x2000,
                    partition_type: PartitionType::OtaData,
                },
                Partition {
                    name: "ota_0".into(),
                    offset: 0x10000,
                    size: 0x1C0000,
                    partition_type: PartitionType::OtaOld,
                },
                Partition {
                    name: "ota_1".into(),
                    offset: 0x1D0000,
                    size: 0x1C0000,
                    partition_type: PartitionType::OtaNew,
                },
                Partition {
                    name: "spiffs".into(),
                    offset: 0x390000,
                    size: 0x70000,
                    partition_type: PartitionType::Spiffs,
                },
            ],
        }
    }

    pub fn list(&self) -> &[Partition] {
        &self.partitions
    }

    pub fn find(&self, name: &str) -> Option<&Partition> {
        self.partitions.iter().find(|p| p.name == name)
    }

    pub fn find_by_type(&self, pt: PartitionType) -> Vec<&Partition> {
        self.partitions
            .iter()
            .filter(|p| p.partition_type == pt)
            .collect()
    }

    pub fn ota_partition(&self) -> Option<&Partition> {
        self.find("ota_1")
    }

    pub fn active_ota(&self) -> Option<&Partition> {
        self.find("ota_0")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_lookup() {
        let mgr = PartitionManager::new();
        assert_eq!(mgr.list().len(), 6);
        assert!(mgr.find("ota_0").is_some());
        assert!(mgr.find("nonexistent").is_none());
        assert_eq!(mgr.ota_partition().unwrap().name, "ota_1");
    }
}
