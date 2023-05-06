use crate::elements::CPFunctionFeatures;
use crate::elements::NodeID;
use crate::elements::RecoveryTimestampIE;
use crate::headers::Header;

#[derive(Debug, Default)]
pub struct AssociationSetup {
    pub header: Header,
    pub node_id: NodeID,                //make option
    pub timestamp: RecoveryTimestampIE, //make option
    pub cp: CPFunctionFeatures,
}

impl AssociationSetup {
    pub fn new() -> AssociationSetup {
        let mut message: AssociationSetup = AssociationSetup {
            header: Header::New(5, None, None),
            node_id: NodeID::new(0, vec![17, 0, 0, 1]),
            timestamp: RecoveryTimestampIE::new(12),
            cp: CPFunctionFeatures::new(),
        }; //ieler optional olacak ona gore
        message.header.lenght += (message.node_id.ie_lenght
            + 4
            + message.timestamp.ie_lenght
            + 4
            + message.cp.ie_lenght
            + 4);
        message
    }

    pub fn to_bytes(mut self) -> Vec<u8> {
        let mut res: Vec<u8> = vec![];
        res.extend(self.header.to_bytes().iter());
        res.append(&mut self.node_id.to_bytes());
        res.extend(self.timestamp.to_bytes().iter());
        res.append(&mut self.cp.to_bytes());
        res
    }
}
