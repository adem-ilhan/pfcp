use std::vec;

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
            _ => println!(""),
        };
        node
    }

    pub fn to_bytes(mut self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        res.append(&mut self.ie_type.to_be_bytes().to_vec());
        res.append(&mut self.ie_lenght.to_be_bytes().to_vec());
        res.push(self.node_type);
        res.append(&mut self.node_value);
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

//f_teid 21
pub struct F_Teid {
    pub ie_type: u16,
    pub ie_lenght: u16,
    pub flags: u8,
    pub teid: Option<u32>,
    pub ipv4: Option<Vec<u8>>,
    pub ipv6: Option<String>,
    pub choose: Option<u8>,
}

impl F_Teid {
    pub fn new(flags: u8) -> F_Teid {
        let mut f_teid: F_Teid = F_Teid {
            ie_type: 21,
            ie_lenght: 0,
            flags: flags,
            teid: None,
            ipv4: None, //Some(vec![127, 0, 0, 1]),
            ipv6: None,
            choose: None,
        };
        f_teid.ie_lenght = 1;
        if f_teid.teid.is_some() {
            f_teid.ie_lenght += 4;
        }
        if f_teid.ipv4.is_some() {
            f_teid.ie_lenght += 4;
        }
        if f_teid.ipv6.is_some() {
            f_teid.ie_lenght += 16;
        }
        if f_teid.choose.is_some() {
            f_teid.ie_lenght += 1;
        }
        f_teid
    }
    pub fn to_bytes(mut self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        res.append(&mut self.ie_type.to_be_bytes().to_vec());
        res.append(&mut self.ie_lenght.to_be_bytes().to_vec());
        res.push(self.flags);
        if self.teid.is_some() {
            res.append(&mut self.teid.unwrap().to_be_bytes().to_vec());
        };
        if self.ipv4.is_some() {
            res.append(&mut self.ipv4.unwrap());
        }
        if self.ipv6.is_some() {
            res.append(&mut self.ipv6.unwrap().into_bytes().to_vec());
        }
        if self.choose.is_some() {
            res.push(self.choose.unwrap());
        };

        res
    }
}

pub struct OuterHeaderRemoval {
    pub ie_type: u16,
    pub ie_lenght: u16,
    pub description: u8,
    pub deletion: u8,
}

impl OuterHeaderRemoval {
    pub fn new() -> OuterHeaderRemoval {
        let mut outerheader: OuterHeaderRemoval = OuterHeaderRemoval {
            ie_type: 95,
            ie_lenght: 2,
            description: 0,
            deletion: 1,
        };
        outerheader
    }

    pub fn to_bytes(mut self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        res.append(&mut self.ie_type.to_be_bytes().to_vec());
        res.append(&mut self.ie_lenght.to_be_bytes().to_vec());
        res.push(self.description);
        res.push(self.deletion);
        res
    }
}

pub struct FarId {
    pub ie_type: u16,
    pub ie_lenght: u16,
    pub value: u32,
}

impl FarId {
    pub fn new(id: u32) -> FarId {
        let mut farid: FarId = FarId {
            ie_type: 108,
            ie_lenght: 4,
            value: id,
        };
        farid
    }
    pub fn to_bytes(mut self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        res.append(&mut self.ie_type.to_be_bytes().to_vec());
        res.append(&mut self.ie_lenght.to_be_bytes().to_vec());
        res.append(&mut self.value.to_be_bytes().to_vec());

        res
    }
}

pub struct UrrId {
    pub ie_type: u16,
    pub ie_lenght: u16,
    pub value: u32,
}

impl UrrId {
    pub fn new(id: u32) -> UrrId {
        let mut urrid: UrrId = UrrId {
            ie_type: 81,
            ie_lenght: 4,
            value: id,
        };
        urrid
    }
    pub fn to_bytes(mut self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        res.append(&mut self.ie_type.to_be_bytes().to_vec());
        res.append(&mut self.ie_lenght.to_be_bytes().to_vec());
        res.append(&mut self.value.to_be_bytes().to_vec());

        res
    }
}

pub struct QerId {
    pub ie_type: u16,
    pub ie_lenght: u16,
    pub value: u32,
}

impl QerId {
    pub fn new(id: u32) -> QerId {
        let mut qerid: QerId = QerId {
            ie_type: 109,
            ie_lenght: 4,
            value: id,
        };
        qerid
    }
    pub fn to_bytes(mut self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        res.append(&mut self.ie_type.to_be_bytes().to_vec());
        res.append(&mut self.ie_lenght.to_be_bytes().to_vec());
        res.append(&mut self.value.to_be_bytes().to_vec());

        res
    }
}

pub struct ApplyAction {
    pub ie_type: u16,
    pub ie_lenght: u16,
    pub flags: u16,
}

impl ApplyAction {
    pub fn new(flags: u16) -> ApplyAction {
        let mut action: ApplyAction = ApplyAction {
            ie_type: 44,
            ie_lenght: 2,
            flags: flags,
        };
        action
    }
    pub fn to_bytes(mut self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        res.append(&mut self.ie_type.to_be_bytes().to_vec());
        res.append(&mut self.ie_lenght.to_be_bytes().to_vec());
        res.append(&mut self.flags.to_be_bytes().to_vec());
        res
    }
}
pub struct ActivatePredefinedRule {
    pub ie_type: u16,
    pub ie_lenght: u16,
    pub rule: String,
}

impl ActivatePredefinedRule {
    pub fn new() -> ActivatePredefinedRule {
        let mut rule: ActivatePredefinedRule = ActivatePredefinedRule {
            ie_type: 106,
            ie_lenght: 0,
            rule: ("testes".to_string()),
        };
        rule.ie_lenght = rule.rule.len() as u16;
        rule
    }
    pub fn to_bytes(self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        res.append(&mut self.ie_type.to_be_bytes().to_vec());
        res.append(&mut self.ie_lenght.to_be_bytes().to_vec());
        res.append(&mut self.rule.into_bytes().to_vec());
        res
    }
}
//Grouped IES
//PDI
pub struct PDI {
    pub ie_type: u16,
    pub ie_lenght: u16,
    pub source: SourceInterface,
    pub f_teid: F_Teid,
    //rest of ies
}

impl PDI {
    pub fn new(sourceinterface: u8) -> PDI {
        let mut pdi: PDI = PDI {
            ie_type: 2,
            ie_lenght: 10,
            source: SourceInterface::new(sourceinterface),
            f_teid: F_Teid::new(4),
        };

        pdi
    }

    pub fn to_bytes(mut self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        res.append(&mut self.ie_type.to_be_bytes().to_vec());
        res.append(&mut self.ie_lenght.to_be_bytes().to_vec());
        res.append(&mut self.source.to_bytes());
        res.append(&mut self.f_teid.to_bytes());
        res
    }
}

pub struct DestinationInterfae {
    pub ie_type: u16,
    pub ie_lenght: u16,
    pub destination: u8,
}

impl DestinationInterfae {
    pub fn new(dst: u8) -> DestinationInterfae {
        let mut interface: DestinationInterfae = DestinationInterfae {
            ie_type: 42,
            ie_lenght: 1,
            destination: dst,
        };
        interface
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        res.append(&mut self.ie_type.to_be_bytes().to_vec());
        res.append(&mut self.ie_lenght.to_be_bytes().to_vec());
        res.push(self.destination);
        res
    }
}
pub struct ForwardingParameters {
    pub ie_type: u16,
    pub ie_lenght: u16,
    pub destination: DestinationInterfae,
}
impl ForwardingParameters {
    pub fn new() -> ForwardingParameters {
        let mut fwdparams: ForwardingParameters = ForwardingParameters {
            ie_type: 4,
            ie_lenght: 0,
            destination: DestinationInterfae::new(1),
        };
        fwdparams.ie_lenght = 4 + fwdparams.destination.ie_lenght;
        fwdparams
    }
    pub fn to_bytes(self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        res.append(&mut self.ie_type.to_be_bytes().to_vec());
        res.append(&mut self.ie_lenght.to_be_bytes().to_vec());
        res.append(&mut self.destination.to_bytes());
        res
    }
}
pub struct CreatePDR {
    pub ie_type: u16,
    pub ie_lenght: u16,
    pub recedence: Precedence,
    pub pdrid: PdrId,
    pub pdi: PDI,
    pub outerheaderrenoval: OuterHeaderRemoval,
    pub farid: FarId,
    pub urrid: UrrId,
    pub qerid: QerId,
    pub activaterule: ActivatePredefinedRule,
}

impl CreatePDR {
    pub fn new() -> CreatePDR {
        let mut pdr: CreatePDR = CreatePDR {
            ie_type: 1,
            ie_lenght: 0,
            recedence: Precedence::new(1),
            pdrid: PdrId::new(1),
            pdi: PDI::new(1),
            outerheaderrenoval: OuterHeaderRemoval::new(),
            farid: FarId::new(1),
            urrid: UrrId::new(1),
            qerid: QerId::new(1),
            activaterule: ActivatePredefinedRule::new(),
        };
        pdr.ie_lenght += 32//total ie type and ie lenght lenght
            + pdr.recedence.ie_lenght
            + pdr.pdrid.ie_lenght
            + pdr.pdi.ie_lenght
            + pdr.outerheaderrenoval.ie_lenght
            + pdr.farid.ie_lenght
            + pdr.urrid.ie_lenght
            + pdr.qerid.ie_lenght
            + pdr.activaterule.ie_lenght;

        pdr
    }
    pub fn to_bytes(self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();

        res.append(&mut self.ie_type.to_be_bytes().to_vec());
        res.append(&mut self.ie_lenght.to_be_bytes().to_vec());
        res.append(&mut self.pdrid.to_bytes());
        res.append(&mut self.recedence.to_bytes());
        res.append(&mut self.pdi.to_bytes());
        res.append(&mut self.outerheaderrenoval.to_bytes());
        res.append(&mut self.farid.to_bytes());
        res.append(&mut self.urrid.to_bytes());
        res.append(&mut self.qerid.to_bytes());
        res.append(&mut self.activaterule.to_bytes());
        res
    }
}

pub struct CreateFar {
    pub ie_type: u16,
    pub ie_lenght: u16,
    pub farid: FarId,
    pub applyaction: ApplyAction,
    pub forwardingparams: ForwardingParameters,
}

impl CreateFar {
    pub fn new() -> CreateFar {
        let mut far: CreateFar = CreateFar {
            ie_type: 3,
            ie_lenght: 0,
            farid: FarId::new(1),
            applyaction: ApplyAction::new(65280),
            forwardingparams: ForwardingParameters::new(),
        };
        far.ie_lenght =
            12 + far.farid.ie_lenght + far.applyaction.ie_lenght + far.forwardingparams.ie_lenght;
        far
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        res.append(&mut self.ie_type.to_be_bytes().to_vec());
        res.append(&mut self.ie_lenght.to_be_bytes().to_vec());
        res.append(&mut self.farid.to_bytes());
        res.append(&mut self.applyaction.to_bytes());
        res.append(&mut self.forwardingparams.to_bytes());
        res
    }
}
