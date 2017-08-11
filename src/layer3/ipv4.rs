use layer4::{TransportProtocol,TransportProtocols,TransportProtocolNotRecognizedError};

pub fn parse_protocol_field(proto_num: u8) -> Result<TransportProtocol,TransportProtocolNotRecognizedError> {
    TransportProtocols(proto_num).cloned().ok_or(TransportProtocolNotRecognizedError(proto_num))
}
