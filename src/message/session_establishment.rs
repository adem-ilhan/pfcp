use crate::elements::CreateFar;
use crate::elements::CreatePDR;
use crate::elements::F_Seid;
use crate::elements::NodeID;
use crate::headers::Header;
pub struct SessionEstablishment {
    pub header: Header, //50
    pub nodeid: NodeID,
    pub fseid: F_Seid,
    pub pdr: CreatePDR,
    pub far: CreateFar,
}

impl SessionEstablishment {
    pub fn new() -> SessionEstablishment {
        let mut message: SessionEstablishment = SessionEstablishment {
            header: Header::New(50, Some(123456789), Some(10)),
            nodeid: NodeID::new(0, vec![17, 0, 0, 1]),
            fseid: F_Seid::new(),
            pdr: CreatePDR::new(),
            far: CreateFar::new(),
        };
        message.header.flag = 33;
        message.header.lenght += message.nodeid.ie_lenght
            + 4
            + message.fseid.ie_lenght
            + 4
            + message.pdr.ie_lenght
            + 4
            + message.far.ie_lenght
            + 4;
        message
    }
    pub fn to_bytes(self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();

        res.append(&mut self.header.to_bytes());
        res.append(&mut self.nodeid.to_bytes());
        res.append(&mut self.fseid.to_bytes());
        res.append(&mut self.pdr.to_bytes());
        res.append(&mut self.far.to_bytes());
        res
    }
}
