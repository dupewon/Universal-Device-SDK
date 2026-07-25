const CRC16_CCITT_TABLE: [u16; 256] = {
    let mut table = [0u16; 256];
    let mut i = 0u16;
    loop {
        let mut crc = i << 8;
        let mut j = 0;
        loop {
            if crc & 0x8000 != 0 {
                crc = (crc << 1) ^ 0x1021;
            } else {
                crc <<= 1;
            }
            j += 1;
            if j == 8 {
                break;
            }
        }
        table[i as usize] = crc;
        i += 1;
        if i == 256 {
            break;
        }
    }
    table
};

pub fn crc16_ccitt(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;
    for &byte in data {
        let idx = ((crc >> 8) ^ byte as u16) & 0xFF;
        crc = (crc << 8) ^ CRC16_CCITT_TABLE[idx as usize];
    }
    crc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc16_known() {
        let crc = crc16_ccitt(b"123456789");
        assert_eq!(crc, 0x29B1);
    }

    #[test]
    fn test_crc16_empty() {
        let crc = crc16_ccitt(b"");
        assert_eq!(crc, 0xFFFF);
    }
}
