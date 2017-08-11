pub mod ipv4;

use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct NetworkProtocolNotRecognizedError(pub u16);

impl NetworkProtocolNotRecognizedError {
    pub fn new(val: u16) -> NetworkProtocolNotRecognizedError {
        NetworkProtocolNotRecognizedError(val)
    }
}

impl Error for NetworkProtocolNotRecognizedError {
    fn description(&self) -> &str {
        "Supplied NetworkProtocol is not recognized."
    }
}

impl fmt::Display for NetworkProtocolNotRecognizedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	    write!(f,"Supplied NetworkProtocol was not recognized. NetworkProtocol: {:04X}", self.0)
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NetworkProtocol {
    IPv4,
    ARP,
    Wake_on_LAN,
    Reverse_ARP,
    AppleTalk,
    AppleTalk_ARP,
    VLAN_Tagged,
    IPX,
    IPv6,
    Hyper_SCSI,
    ATAoE,
    LLDP,
    MAC_Sec,
    FCoE,
    Double_VLAN_Tagged,
}

impl fmt::Display for NetworkProtocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self,f)
    }
}
