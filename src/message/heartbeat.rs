use crate::elements::RecoveryTimestampIE;
use crate::headers::Header;
#[derive(Debug, Default)]
pub struct HeartbeatRequest {
    pub header: Header,
    pub recovery_time_stamp: RecoveryTimestampIE,
}

impl HeartbeatRequest {
    // New function
    pub fn new() -> HeartbeatRequest {
        let mut m: HeartbeatRequest = HeartbeatRequest {
            header: Header::New(1, None, None),
            recovery_time_stamp: RecoveryTimestampIE::new(15),
        };

        // m.header.set_length()
        m.header.lenght += 8;
        m
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = vec![];
        bytes.extend(self.header.to_bytes().iter());
        bytes.extend(self.recovery_time_stamp.to_bytes().iter());
        bytes
    }
}
