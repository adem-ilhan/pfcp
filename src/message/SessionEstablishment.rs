use crate::elements::CPFunctionFeatures;
use crate::elements::NodeID;
use crate::elements::RecoveryTimestampIE;
use crate::headers::Header;

pub struct SessionEstablishment {
    pub header: Header, //50
    pub nodeid: NodeID,
}
