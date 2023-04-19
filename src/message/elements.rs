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
            ie_lenght: 8,
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

        if let Some(ipv4) = &mut self.ipv4 {
            res.append(ipv4);
        }

        unsafe {
            if let Some(ipv6) = &mut self.ipv6 {
                res.append(ipv6.as_mut_vec()); //hata
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
