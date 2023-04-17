#[derive(Debug, Default)]
pub struct Header {
    pub flag: u8,
    pub msg_type: u8,
    pub lenght: u16,
    pub seid: Option<u32>,
    pub seq: u32,
    pub priority: Option<u8>,
}

impl Header {
    pub fn New(t: u8, seid: Option<u32>, pri: Option<u8>) -> Header {
        let mut m: Header = Header {
            flag: 32,
            msg_type: t,
            lenght: 0,
            seid: seid,
            seq: 1,
            priority: pri,
        };
        m.set_length();
        m
    }
    pub fn set_length(&mut self) {
        self.lenght = 4;
        if !self.seid.is_none() {
            self.lenght + 8;
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.push(self.flag);
        bytes.push(self.msg_type);
        bytes.extend_from_slice(&self.lenght.to_be_bytes());
        if let Some(seid) = self.seid {
            bytes.extend_from_slice(&seid.to_be_bytes());
        }
        bytes.extend_from_slice(&self.seq.to_be_bytes());
        if let Some(priority) = self.priority {
            bytes.push(priority);
        }
        bytes
    }
}
