extern crate phf;

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ICMPControlMessageNotRecognizedError(pub u16);

impl ICMPControlMessageNotRecognizedError {
    pub fn new(val: u16) -> ICMPControlMessageNotRecognizedError {
        ICMPControlMessageNotRecognizedError(val)
    }
}

impl Error for ICMPControlMessageNotRecognizedError {
    fn description(&self) -> &str {
        "Supplied ICMP Control Message is not recognized."
    }
}

impl fmt::Display for ICMPControlMessageNotRecognizedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	    write!(f,"Supplied ICMP Control Message was not recognized. Number: {:04X}", self.0)
    }
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, PartialEq)]
pub enum ICMPControlMessage {
    ECHO_REPLY,
    DEST_UNREACHABLE_NET_UNREACHABLE,
    DEST_UNREACHABLE_HOST_UNREACHABLE,
    DEST_UNREACHABLE_PROTO_UNREACHABLE,
    DEST_UNREACHABLE_PORT_UNREACHABLE,
    DEST_UNREACHABLE_FRAG_BUT_DONT_FRAG,
    DEST_UNREACHABLE_SRC_ROUTE_FAIL,
    DEST_UNREACHABLE_DEST_NET_UNKNOWN,
    DEST_UNREACHABLE_DEST_HOST_UNKNOWN,
    DEST_UNREACHABLE_SRC_HOST_ISOLATED,
    DEST_UNREACHABLE_DEST_NET_ADMIN_DENIED,
    DEST_UNREACHABLE_DEST_HOST_ADMIN_DENIED,
    DEST_UNREACHABLE_DEST_NET_TOS_UNREACHABLE,
    DEST_UNREACHABLE_DEST_HOST_TOS_UNREACHABLE,
    DEST_UNREACHABLE_COMM_ADMIN_DENIED,
    DEST_UNREACHABLE_HOST_PRECEDENCE_VIOLATION,
    DEST_UNREACHABLE_PRECEDENCE_CUTOFF,
    SOURCE_QUENCH,
    REDIRECT_FOR_NET,
    REDIRECT_FOR_HOST,
    REDIRECT_FOR_TOS_AND_NET,
    REDIRECT_FOR_TOS_AND_HOST,
    ECHO,
    ROUTER_ADVERT_NORMAL,
    ROUTER_ADVERT_NOT_COMMON,
    ROUTER_SOLICIT,
    TIME_EXCEEDED_TTL_IN_TRANSIT,
    TIME_EXCEEDED_FRAG_REASSY_TIME,
    PARAM_PROBLEM_PTR_ERROR,
    PARAM_PROBLEM_MISSING_REQ_OPT,
    PARAM_PROBLEM_BAD_LENGTH,
    TIMESTAMP,
    TIMESTAMP_REPLY,
    INFO_REQUEST,
    INFO_REPLY,
    PHOTURIS_BAD_SPI,
    PHOTURIS_AUTHENTICATION_FAIL,
    PHOTURIS_DECOMPRESS_FAIL,
    PHOTURIS_DECRYPTION_FAIL,
    PHOTURIS_NEED_AUTHENTICATION,
    PHOTURIS_NEED_AUTHORIZATION,
}

impl fmt::Display for ICMPControlMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self,f)
    }
}

// Defined by IANA as "Internet Control Message Protocol (ICMP) Parameters":
// https://www.iana.org/assignments/icmp-parameters/icmp-parameters.xhtml
u16_key_phf_map! {
    ICMPControlMessages -> ICMPControlMessage = {
	    0x0000 => ICMPControlMessage::ECHO_REPLY,
	    0x0300 => ICMPControlMessage::DEST_UNREACHABLE_NET_UNREACHABLE,
	    0x0301 => ICMPControlMessage::DEST_UNREACHABLE_HOST_UNREACHABLE,
	    0x0302 => ICMPControlMessage::DEST_UNREACHABLE_PROTO_UNREACHABLE,
	    0x0303 => ICMPControlMessage::DEST_UNREACHABLE_PORT_UNREACHABLE,
	    0x0304 => ICMPControlMessage::DEST_UNREACHABLE_FRAG_BUT_DONT_FRAG,
	    0x0305 => ICMPControlMessage::DEST_UNREACHABLE_SRC_ROUTE_FAIL,
	    0x0306 => ICMPControlMessage::DEST_UNREACHABLE_DEST_NET_UNKNOWN,
	    0x0307 => ICMPControlMessage::DEST_UNREACHABLE_DEST_HOST_UNKNOWN,
	    0x0308 => ICMPControlMessage::DEST_UNREACHABLE_SRC_HOST_ISOLATED,
	    0x0309 => ICMPControlMessage::DEST_UNREACHABLE_DEST_NET_ADMIN_DENIED,
	    0x030A => ICMPControlMessage::DEST_UNREACHABLE_DEST_HOST_ADMIN_DENIED,
	    0x030B => ICMPControlMessage::DEST_UNREACHABLE_DEST_NET_TOS_UNREACHABLE,
	    0x030C => ICMPControlMessage::DEST_UNREACHABLE_DEST_HOST_TOS_UNREACHABLE,
        0x030D => ICMPControlMessage::DEST_UNREACHABLE_COMM_ADMIN_DENIED,
        0x030E => ICMPControlMessage::DEST_UNREACHABLE_HOST_PRECEDENCE_VIOLATION,
        0x030F => ICMPControlMessage::DEST_UNREACHABLE_PRECEDENCE_CUTOFF,
	    0x0400 => ICMPControlMessage::SOURCE_QUENCH,
	    0x0500 => ICMPControlMessage::REDIRECT_FOR_NET,
	    0x0501 => ICMPControlMessage::REDIRECT_FOR_HOST,
	    0x0502 => ICMPControlMessage::REDIRECT_FOR_TOS_AND_NET,
	    0x0503 => ICMPControlMessage::REDIRECT_FOR_TOS_AND_HOST,
	    0x0800 => ICMPControlMessage::ECHO,
	    0x0900 => ICMPControlMessage::ROUTER_ADVERT_NORMAL,
	    0x0910 => ICMPControlMessage::ROUTER_ADVERT_NOT_COMMON,
	    0x0A00 => ICMPControlMessage::ROUTER_SOLICIT,
	    0x0B00 => ICMPControlMessage::TIME_EXCEEDED_TTL_IN_TRANSIT,
	    0x0B01 => ICMPControlMessage::TIME_EXCEEDED_FRAG_REASSY_TIME,
	    0x0C00 => ICMPControlMessage::PARAM_PROBLEM_PTR_ERROR,
	    0x0C01 => ICMPControlMessage::PARAM_PROBLEM_MISSING_REQ_OPT,
        0x0C02 => ICMPControlMessage::PARAM_PROBLEM_BAD_LENGTH,
        0x0D00 => ICMPControlMessage::TIMESTAMP,
        0x0E00 => ICMPControlMessage::TIMESTAMP_REPLY,
	    0x0F00 => ICMPControlMessage::INFO_REQUEST,
	    0x1000 => ICMPControlMessage::INFO_REPLY,
	    0x2800 => ICMPControlMessage::PHOTURIS_BAD_SPI,
	    0x2801 => ICMPControlMessage::PHOTURIS_AUTHENTICATION_FAIL,
	    0x2802 => ICMPControlMessage::PHOTURIS_DECOMPRESS_FAIL,
	    0x2803 => ICMPControlMessage::PHOTURIS_DECRYPTION_FAIL,
	    0x2804 => ICMPControlMessage::PHOTURIS_NEED_AUTHENTICATION,
	    0x2805 => ICMPControlMessage::PHOTURIS_NEED_AUTHORIZATION,
    }                               
}

pub fn parse_icmp_control_msg(msg: u16) -> Result<ICMPControlMessage,ICMPControlMessageNotRecognizedError> {
    ICMPControlMessages(msg).cloned().ok_or(ICMPControlMessageNotRecognizedError(msg))
}
