#[derive(Debug, Default)]
pub struct Header {
    pub flag: u8,
    pub msg_type: u8,
    pub lenght: u16,
    pub seid: Option<u64>,
    pub spare: u8,
    pub seq: Vec<u8>,
    pub priority: Option<u8>,
}

impl Header {
    pub fn New(t: u8, seid: Option<u64>, pri: Option<u8>) -> Header {
        let mut m: Header = Header {
            flag: 35,
            msg_type: t,
            lenght: 0,
            seid: seid,
            spare: 0,
            seq: vec![1, 3, 4],
            priority: pri,
        };
        m.set_length();
        m
    }
    pub fn set_length(&mut self) {
        self.lenght = 4;
        if !self.seid.is_none() {
            self.lenght += 8;
        }
    }

    pub fn to_bytes(mut self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new(); //vec![
                                             //     0x21, 0x32, 0x00, 0x41, 0x00, 0x00, 0x00, 0x00, 0x07, 0x5b, 0xcd, 0x15, 0x5b, 0xcd,
                                             //     0x15, 0x00,
                                             // ];
        bytes.push(self.flag);
        bytes.push(self.msg_type);
        bytes.extend_from_slice(&self.lenght.to_be_bytes());
        println!("{:?}", self.lenght.to_be_bytes());
        //
        if let Some(seid) = self.seid {
            bytes.extend_from_slice(&seid.to_be_bytes());
        }
        //
        bytes.append(&mut self.seq);
        if let Some(priority) = self.priority {
            bytes.push(priority);
        }

        bytes
    }
}
