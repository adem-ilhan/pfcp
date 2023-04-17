use byteorder::{BigEndian, ByteOrder};
#[derive(Debug)]
pub struct RecoveryTimestampIE {
    pub ie_type: u16,
    pub ie_lenght: u16,
    pub timestamp: u64,
}

impl RecoveryTimestampIE {
    pub fn new(timestamp: u64) -> RecoveryTimestampIE {
        RecoveryTimestampIE {
            ie_type: 96,
            ie_lenght: 4,
            timestamp,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.extend_from_slice(&self.ie_type.to_be_bytes());
        res.extend_from_slice(&self.ie_lenght.to_be_bytes());
        res.extend_from_slice(&self.timestamp.to_be_bytes());
        res
    }
}

impl Default for RecoveryTimestampIE {
    fn default() -> Self {
        RecoveryTimestampIE::new(0)
    }
}
