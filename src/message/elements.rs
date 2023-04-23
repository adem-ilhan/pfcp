// Recovery Time Stamp
#[derive(Debug, Default, Clone, Copy)]
pub struct RecoveryTimestampIE {
    pub ie_type: u16,
    pub ie_lenght: u16,
    pub timestamp: u32,
}

impl RecoveryTimestampIE {
    pub fn new(timestamp: u64) -> RecoveryTimestampIE {
        RecoveryTimestampIE {
            ie_type: 96,
            ie_lenght: 4,
            timestamp: 14 as u32,
        }
    }

    pub fn to_bytes(mut self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        res.append(&mut self.ie_type.to_be_bytes().to_vec());
        res.append(&mut self.ie_lenght.to_be_bytes().to_vec());
        res.append(&mut self.timestamp.to_le_bytes().to_vec());

        res
    }
}

// Node ID
pub const NODE_ID_TYPE_IPV4: u8 = 0;
pub const NODE_ID_TYPE_IPV6: u8 = 1;
pub const NODE_ID_TYPE_FQDN: u8 = 2;

#[derive(Debug, Default)]
pub struct NodeID {
    pub ie_type: u16,
    pub ie_lenght: u16,
    pub node_type: u8,
    pub node_value: Vec<u8>,
}

impl NodeID {
    pub fn new(n_type: u8, n_value: Vec<u8>) -> NodeID {
        let mut node: NodeID = NodeID {
            ie_type: 60,
            ie_lenght: 1,
            node_type: n_type,
            node_value: n_value,
        };
        match n_type {
            NODE_ID_TYPE_IPV4 => node.ie_lenght += 4,
            NODE_ID_TYPE_IPV6 => node.ie_lenght += 16,
            NODE_ID_TYPE_FQDN => node.ie_lenght += node.node_value.len() as u16,
            _ => println!("Ain't special"),
        };
        node
    }

    pub fn to_bytes(mut self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        res.append(&mut self.ie_type.to_be_bytes().to_vec());
        res.append(&mut self.ie_lenght.to_be_bytes().to_vec());
        res.push(self.node_type);
        res.append(&mut self.node_value);
        println!("{:?}", res);
        res
    }
}

//CP Fuction Feautures

#[derive(Debug, Default)]
pub struct CPFunctionFeatures {
    pub ie_type: u16,
    pub ie_lenght: u16,
    pub features: u8,
}

impl CPFunctionFeatures {
    pub fn new() -> CPFunctionFeatures {
        let mut cp: CPFunctionFeatures = CPFunctionFeatures {
            ie_type: 43,
            ie_lenght: 1,
            features: 1,
        };
        cp
    }

    pub fn to_bytes(mut self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        res.append(&mut self.ie_type.to_be_bytes().to_vec());
        res.append(&mut self.ie_lenght.to_be_bytes().to_vec());
        res.push(self.features);
        res
    }
}

//F-SEID
pub struct F_Seid {
    pub ie_type: u16,
    pub ie_lenght: u16,
    pub flags: u8,
    pub seid: u64,
    pub ipv4: Option<Vec<u8>>,
    pub ipv6: Option<String>,
}

impl F_Seid {
    pub fn new() -> F_Seid {
        let mut fseid: F_Seid = F_Seid {
            ie_type: 57,
            ie_lenght: 13,
            seid: 1234567,
            flags: 2,
            ipv4: Some(vec![11, 0, 0, 1]),
            ipv6: None,
        };
        fseid
    }

    pub fn to_bytes(mut self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        res.append(&mut self.ie_type.to_be_bytes().to_vec());
        res.append(&mut self.ie_lenght.to_be_bytes().to_vec());
        res.push(self.flags);
        res.append(&mut self.seid.to_be_bytes().to_vec());
        if self.ipv4.is_some() {
            match self.ipv4 {
                Some(mut X) => res.append(&mut X),
                _ => print!("noth"),
            }
        }

        res
    }
}

//PDR ID//56

pub struct PdrId {
    pub ie_type: u16,
    pub ie_lenght: u16,
    pub rule_id: u16,
}

impl PdrId {
    pub fn new(id: u16) -> PdrId {
        let mut pdr: PdrId = PdrId {
            ie_type: 56,
            ie_lenght: 2,
            rule_id: id,
        };
        pdr
    }

    pub fn to_bytes(mut self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        res.append(&mut self.ie_type.to_be_bytes().to_vec());
        res.append(&mut self.ie_lenght.to_be_bytes().to_vec());
        res.append(&mut self.rule_id.to_be_bytes().to_vec());

        res
    }
}

//Precedence

pub struct Precedence {
    pub ie_type: u16,
    pub ie_lenght: u16,
    pub pre: u32,
}

impl Precedence {
    pub fn new(pre: u32) -> Precedence {
        let mut ie: Precedence = Precedence {
            ie_type: 29,
            ie_lenght: 4,
            pre: pre,
        };
        ie
    }

    pub fn to_bytes(mut self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        res.append(&mut self.ie_type.to_be_bytes().to_vec());
        res.append(&mut self.ie_lenght.to_be_bytes().to_vec());
        res.append(&mut self.pre.to_be_bytes().to_vec());

        res
    }
}

//Source Interface 20

pub struct SourceInterface {
    pub ie_type: u16,
    pub ie_lenght: u16,
    pub interface: u8,
}

impl SourceInterface {
    pub fn new(interface: u8) -> SourceInterface {
        let mut source: SourceInterface = SourceInterface {
            ie_type: 20,
            ie_lenght: 1,
            interface: interface,
        };
        source
    }

    pub fn to_bytes(mut self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        res.append(&mut self.ie_type.to_be_bytes().to_vec());
        res.append(&mut self.ie_lenght.to_be_bytes().to_vec());
        res.push(self.interface);
        res
    }
}

//PDI
pub struct PDI {
    pub ie_type: u16,
    pub ie_lenght: u16,
    pub source: SourceInterface,
    //rest of ies
}

impl PDI {
    pub fn new(sourceinterface: u8) -> PDI {
        let mut pdi: PDI = PDI {
            ie_type: 2,
            ie_lenght: 5,
            source: SourceInterface::new(sourceinterface),
        };

        pdi
    }

    pub fn to_bytes(mut self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        res.append(&mut self.ie_type.to_be_bytes().to_vec());
        res.append(&mut self.ie_lenght.to_be_bytes().to_vec());
        res.append(&mut self.source.to_bytes());
        res
    }
}

pub struct CreatePDR {
    pub ie_type: u16,
    pub ie_lenght: u16,
    pub recedence: Precedence,
    pub pdrid: PdrId,
    pub pdi: PDI,
}

impl CreatePDR {
    pub fn new() -> CreatePDR {
        let mut pdr: CreatePDR = CreatePDR {
            ie_type: 1,
            ie_lenght: 0,
            recedence: Precedence::new(1),
            pdrid: PdrId::new(1),
            pdi: PDI::new(1),
        };
        pdr.ie_lenght += 12 + pdr.recedence.ie_lenght + pdr.pdrid.ie_lenght + pdr.pdi.ie_lenght;
        println!("lenght pdi;{}", pdr.pdi.ie_lenght);

        println!("lenght create pdr;{}", pdr.ie_lenght);
        pdr
    }
    pub fn to_bytes(mut self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();

        res.append(&mut self.ie_type.to_be_bytes().to_vec());
        res.append(&mut self.ie_lenght.to_be_bytes().to_vec());
        res.append(&mut self.pdrid.to_bytes());
        res.append(&mut self.recedence.to_bytes());
        res.append(&mut self.pdi.to_bytes());
        res
    }
}
