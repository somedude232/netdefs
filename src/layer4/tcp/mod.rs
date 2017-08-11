use super::TransportProtocol;

#[derive(Debug,Clone,PartialEq)]
pub enum TCPFlag {
    FIN,
    SYN,
    RST,
    PSH,
    ACK,
    URG,
    ECE,
    CWR,
    NS,
}

pub struct TCPPort {
    proto: TransportProtocol,
    port_num: u16,
}

impl TCPPort {
    pub fn new(protocol: TransportProtocol, port_num: u16) -> TCPPort {
        TCPPort { proto: protocol, port_num: port_num }
    }

    pub fn get_proto(&self) -> TransportProtocol {
        self.proto.clone()
    }

    pub fn get_port_num(&self) -> u16 {
        self.port_num
    }
}

pub struct TCPPortRange {
    proto: TransportProtocol,
    port_num1: u16,
    port_num2: u16,
}

impl TCPPortRange {
    pub fn new(proto: TransportProtocol, first_port_num: u16, second_port_num: u16) -> TCPPortRange {
        if first_port_num > second_port_num {
            panic!("Error in creating TCPPortRange: first port number must be smaller than the second.")
        }
        TCPPortRange { proto: proto, port_num1: first_port_num, port_num2: second_port_num }
    }

    pub fn get_protocol(&self) -> TransportProtocol {
        self.proto.clone()
    }

    pub fn get_first_port_num(&self) -> u16 {
        self.port_num1
    }

    pub fn get_second_port_num(&self) -> u16 {
        self.port_num2
    }

    pub fn is_in_range(&self, port: TCPPort) -> bool {
        port.get_proto() == self.proto && port.get_port_num() <= self.port_num2 && port.get_port_num() >= self.port_num1
    }
}

pub fn get_tcp_flag_vec() -> Vec<TCPFlag> {
    vec![TCPFlag::FIN, TCPFlag::SYN, TCPFlag::RST, TCPFlag::PSH, TCPFlag::ACK, TCPFlag::URG, TCPFlag::ECE, TCPFlag::CWR, TCPFlag::NS]
}

pub fn parse_tcp_flags(flags: u16) -> Vec<TCPFlag> {
    let mut result: Vec<TCPFlag> = Vec::new();
    let mut temp: u16 = 1;
    for flag in get_tcp_flag_vec().iter() {
        if flags & temp == temp {
            result.push(flag.clone());
        }
        temp <<= 1;
    }
    result
}
