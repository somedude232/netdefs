pub mod icmp;
pub mod tcp;

use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct TransportProtocolNotRecognizedError(pub u8);

impl TransportProtocolNotRecognizedError {
    pub fn new(val: u8) -> TransportProtocolNotRecognizedError {
        TransportProtocolNotRecognizedError(val)
    }
}

impl Error for TransportProtocolNotRecognizedError {
    fn description(&self) -> &str {
        "Supplied Transport Protocol Number is not recognized."
    }
}

impl fmt::Display for TransportProtocolNotRecognizedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	    write!(f,"Supplied Transport Protocol Number was not recognized. Number: {:02X}", self.0)
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq, Debug)]
pub enum TransportProtocol {
    HOPOPT,
    ICMP,
    IP_in_IP,
    TCP,
    EGP,
    IGP,
    UDP,
    IPv6,
    IPv6_Route,
    IPv6_Frag,
    IPv6_ICMP,
    IPv6_NoNxt,
    IPv6_Opts,
    SCTP,
    FC,
}

impl fmt::Display for TransportProtocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self,f)
    }
}

// Defined by IANA as "Protocol Numbers": https://www.iana.org/assignments/protocol-numbers/protocol-numbers.xhtml
u8_key_phf_map! {
    TransportProtocols -> TransportProtocol = {
	    0x00 => TransportProtocol::HOPOPT,
	    0x01 => TransportProtocol::ICMP,
	    0x04 => TransportProtocol::IP_in_IP,
	    0x06 => TransportProtocol::TCP,
	    0x08 => TransportProtocol::EGP,
	    0x09 => TransportProtocol::IGP,
	    0x11 => TransportProtocol::UDP,
	    0x29 => TransportProtocol::IPv6,
	    0x2B => TransportProtocol::IPv6_Route,
	    0x2C => TransportProtocol::IPv6_Frag,
	    0x3A => TransportProtocol::IPv6_ICMP,
	    0x3B => TransportProtocol::IPv6_NoNxt,
	    0x3C => TransportProtocol::IPv6_Opts,
	    0x84 => TransportProtocol::SCTP,
        0x85 => TransportProtocol::FC,
    }
}

pub fn parse_protocol_raw(proto_num: u8) -> Result<TransportProtocol,TransportProtocolNotRecognizedError> {
    TransportProtocols(proto_num).cloned().ok_or(TransportProtocolNotRecognizedError(proto_num))
}
