#![macro_use]
extern crate phf;
extern crate regex;

use std::error::Error;
use std::net::Ipv4Addr;
use std::fmt;
use std::cmp;
use regex::Regex;

use layer3::{NetworkProtocol,NetworkProtocolNotRecognizedError};

#[derive(Debug)]
pub struct IPMACAssociateError(pub Ipv4Addr, pub MacAddress);

impl IPMACAssociateError {
    pub fn new(ip: Ipv4Addr, mac: MacAddress) -> IPMACAssociateError {
        IPMACAssociateError(ip, mac)
    }
}

impl Error for IPMACAssociateError {
    fn description(&self) -> &str {
        "IP and MAC Address entries could not be combined and replaced. IP or MAC address not found in set."
    }
}

impl fmt::Display for IPMACAssociateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	    write!(f,"IP and MAC Address entries could not be combined and replaced. Both the following IP & MAC addresses must be present in the set to associate them: {} {}", self.0, self.1)
    }
}

#[derive(Clone, Debug, Copy)]
pub struct MacAddress {
    addr: [u8;6],
}

impl MacAddress {

    pub fn from_bytes(bytes: [u8;6]) -> MacAddress {
        MacAddress { addr: bytes }
    }

    pub fn from_slice(slice: &[u8]) -> MacAddress {
        assert!(slice.len() == 6);
        MacAddress { addr: [slice[0],slice[1],slice[2],slice[3],slice[4],slice[5]] }
    }

    pub fn from_vec(vector: Vec<u8>) -> MacAddress {
        MacAddress::from_slice(vector.as_slice())
    }

    pub fn from_str(str: &str) -> MacAddress {
        let pattern = Regex::new(r"^([[:xdigit:]]{2}[:-]){5}([[:xdigit:]]{2})$").unwrap();
        assert!(pattern.is_match(str));
        let bytes = str.split(|c| c == '-' || c == ':').map(|x| u8::from_str_radix(x,16).unwrap()).collect::<Vec<u8>>();
        MacAddress::from_slice(bytes.as_slice())
    }

    pub fn to_bytes(&self) -> [u8;6] {
        self.addr
    }

    pub fn get_oui(&self) -> [u8;3] {
        [self.addr[0], self.addr[1], self.addr[2]]
    }

    pub fn to_eui64(&self) -> Eui64 {
        Eui64 { addr: [self.addr[0], self.addr[1], self.addr[2], 0xFF, 0xFF, self.addr[3], self.addr[4], self.addr[5]] }
    }
}

impl fmt::Display for MacAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let result  = self.addr[1..].iter().fold(String::new(), |acc, &a| acc + &format!(":{:02X}", a));
        write!(f, "{:02X}{}",self.addr[0],result)
    }
}

impl cmp::PartialEq for MacAddress {
    fn eq(&self, other: &MacAddress) -> bool {
        self.to_string() == other.to_string()
    }
}

#[allow(dead_code)]
type Eui48 = MacAddress;

pub struct Eui64 {
    addr: [u8;8],
}

impl Eui64 {

    pub fn from_bytes(bytes: [u8;8]) -> Eui64 {
        Eui64 { addr: bytes }
    }

    pub fn from_slice(slice: &[u8]) -> Eui64 {
        assert!(slice.len() == 8);
        Eui64 { addr: [slice[0],slice[1],slice[2],slice[3],slice[4],slice[5],slice[6],slice[7]] }
    }

    pub fn from_vec(vector: Vec<u8>) -> Eui64 {
        Eui64::from_slice(vector.as_slice())
    }

    pub fn from_mac_address(mac: MacAddress) -> Eui64 {
        mac.to_eui64()
    }

    pub fn from_string(str: &str) -> Eui64 {
        let pattern = Regex::new(r"^([[:xdigit:]]{2}[:-]){7}([[:xdigit:]]{2})$").unwrap();
        assert!(pattern.is_match(str));
        let bytes = str.split(|c| c == '-' || c == ':').map(|x| u8::from_str_radix(x,16).unwrap()).collect::<Vec<u8>>();
        Eui64::from_slice(bytes.as_slice())
    }

    pub fn get_oui(&self) -> [u8;3] {
        [self.addr[0], self.addr[1], self.addr[2]]
    }

    pub fn to_mac_address(&self) -> MacAddress {
        MacAddress { addr: [self.addr[0], self.addr[1], self.addr[2], self.addr[5], self.addr[6], self.addr[7]] }
    }
}


// Defined by IANA as "IEEE 802 Numbers": https://www.iana.org/assignments/ieee-802-numbers/ieee-802-numbers.xhtml
u16_key_phf_map! {
    NetworkProtocols -> NetworkProtocol = {
	    0x0800 => NetworkProtocol::IPv4,
	    0x0806 => NetworkProtocol::ARP,
	    0x0842 => NetworkProtocol::Wake_on_LAN,
	    0x8035 => NetworkProtocol::Reverse_ARP,
	    0x809B => NetworkProtocol::AppleTalk,
	    0x80F3 => NetworkProtocol::AppleTalk_ARP,
	    0x8100 => NetworkProtocol::VLAN_Tagged,
	    0x8137 => NetworkProtocol::IPX,
	    0x86DD => NetworkProtocol::IPv6,
	    0x889A => NetworkProtocol::Hyper_SCSI,
	    0x88A2 => NetworkProtocol::ATAoE,
        0x88A8 => NetworkProtocol::Double_VLAN_Tagged,
	    0x88CC => NetworkProtocol::LLDP,
	    0x88E5 => NetworkProtocol::MAC_Sec,
	    0x8906 => NetworkProtocol::FCoE,
    }
}

pub fn parse_ethertype(etype: u16) -> Result<NetworkProtocol,NetworkProtocolNotRecognizedError> {
    NetworkProtocols(etype).cloned().ok_or(NetworkProtocolNotRecognizedError(etype))
}

/* IN PROGRESS
// NOTE: Ethernet frames are received in Big Endian.
pub struct Frame {
    dst_mac: MacAddress,
    src_mac: MacAddress,
    vlan_tag: Option<VLAN_Tag>,
    double_vlan_tag: Option<VLAN_Tag>,
    ethertype: NetworkProtocol,
    payload: Vec<u8>,
    fcs: FCS,
}

impl Frame {
    pub fn from_bytevec_be(be_bytes: Vec<u8>) -> Frame {
        let byte_count: u8 = 0;
        let he_bytes = be_bytes.iter().map(|x| u8::from_be(x)).collect::<Vec<u8>>();   // he == host endian
        let dst_mac = MacAddress::from_slice(he_bytes[0..6]);
        let src_mac = MacAddress::from_slice(he_bytes[6..12]);
        byte_count += 12;

        let ethertype = parse_ethertype(((be_bytes[12] as u16) << 8) + (be_bytes[13] as u16)).unwrap();
        let &mut vlan_tag = None;
        let &mut second_vlan_tag = None;
        // let &mut third_vlan_tag = None         // HAS SCIENCE GONE TOO FAR?!
        byte_count += 2;

        if ethertype == NetworkProtocol::Double_VLAN_Tagged {
            second_vlan_tag = Some(VLAN_Tag::from_bytes([be_bytes[12], be_bytes[13], be_bytes[14], be_bytes[15]]));
            vlan_tag = Some(VLAN_Tag::from_bytes([be_bytes[16], be_bytes[17], be_bytes[18], be_bytes[19]]));
            ethertype = parse_ethertype(((be_bytes[20] as u16) << 8) + (be_bytes[21] as u16)).unwrap();
            byte_count += 8;
        }
        else if ethertype == NetworkProtocol::VLAN_Tagged {
            vlan_tag = Some(VLAN_Tag::from_bytes([be_bytes[12], be_bytes[13], be_bytes[14], be_bytes[15]]));
            ethertype = parse_ethertype(((be_bytes[16] as u16) << 8) + (be_bytes[17] as u16)).unwrap();
            byte_count += 4;
        }
       
        let (header, data) = he_bytes.split_at(byte_count as usize);
        
        if data.len() < 50 {
            if vlan_tag.is_none() || data.len() < 46 {
                panic!("Ethernet payload too short!");
            }
        }
        
        let (payload, fcs) = data.split_at(data.len()-4);
        
        Frame { dst_mac: dst_mac, src_mac: src_mac, vlan_tag: vlan_tag, double_vlan_tag: second_vlan_tag, ethertype: ethertype, payload: payload, fcs: FCS::from_slice(fcs)}
    }
}

pub struct VLAN_Tag {
    proto_id: u16,
    ctl_info: u16,
}

impl VLAN_Tag {
    pub fn new(etype: u16, ctl: u16) -> VLAN_Tag {
        VLAN_Tag { proto_id: etype, ctl_info: ctl }
    }

    pub fn from_bytes(bytes: [u8;4]) -> VLAN_Tag {
        let proto = ((bytes[0] as u16) << 8) + (bytes[1] as u16);
        let ctl = ((bytes[0] as u16) << 8) + (bytes[1] as u16);
        VLAN_Tag { proto_id: proto, ctl_info: ctl }
    }

    pub fn get_ctl_info(&self) -> u16 {
        *&self.ctl_info
    }

    // Technically 3 bits, but smallest is bool or u8.
    pub fn get_pcp(&self) -> u8 {
        (&self.ctl_info >> 13) as u8
    }

    pub fn get_dei(&self) -> bool {
        (&self.ctl_info & 0x1000) == 0x1000
    }

    pub fn get_vlan_id(&self) -> u16 {
        &self.ctl_info & 0x0FFF
    }
}

pub struct FCS {
    bytes: u32,
}

impl FCS {
    pub fn from_slice(byte_slice: &[u8]) -> FCS {
        assert!(byte_slice.len() == 4);
        let fcs_int: u32 = ((byte_slice[0] as u32) << 24) + ((byte_slice[1] as u32) << 16) + ((byte_slice[2] as u32) << 8) + (byte_slice[3] as u32);
        FCS { bytes: fcs_int }
    }
}

*/

#[derive(Clone, Copy)]
pub struct IpMacCombo {
    ip: Option<Ipv4Addr>,
    mac: Option<MacAddress>,
}

impl IpMacCombo {
    pub fn new(new_ip: Ipv4Addr, new_mac: MacAddress) -> IpMacCombo {
        IpMacCombo { ip: Some(new_ip), mac: Some(new_mac) }
    }

    pub fn from_ip(new_ip: Ipv4Addr) -> IpMacCombo {
        IpMacCombo { ip: Some(new_ip), mac: None }
    }

    pub fn from_mac(new_mac: MacAddress) -> IpMacCombo {
        IpMacCombo { ip: None, mac: Some(new_mac) }
    }

    pub fn has_ip(&self) -> bool {
        self.ip.is_some()
    }

    pub fn has_mac(&self) -> bool {
        self.mac.is_some()
    }

    pub fn get_mac(&self) -> Option<MacAddress> {
        self.mac.clone()
    }

    pub fn get_ip(&self) -> Option<Ipv4Addr> {
        self.ip.clone()
    }

    pub fn add_ip(&mut self, ip: Ipv4Addr) {
        assert!(self.ip.is_none());
        self.ip = Some(ip);
    }

    pub fn add_mac(&mut self, mac: MacAddress) {
        assert!(self.mac.is_none());
        self.mac = Some(mac);
    }
}

impl cmp::PartialEq for IpMacCombo {
    fn eq(&self, other: &IpMacCombo) -> bool {
        if self.get_ip().is_none() == false {
            if self.get_mac().is_none() == false {
                self.get_mac().unwrap() == other.get_mac().unwrap() && self.get_ip().unwrap() == other.get_ip().unwrap()
            }
            else {
                self.get_mac().is_none() == other.get_mac().is_none() && self.get_ip().unwrap() == other.get_ip().unwrap()
            }
        }
        else {
            if self.get_mac().is_none() == false {
                self.get_mac().unwrap() == other.get_mac().unwrap() && self.get_ip().is_none() == other.get_ip().is_none()
            }
            else {
                self.get_mac().is_none() == other.get_mac().is_none() && self.get_ip().is_none() == other.get_ip().is_none()
            }
        }
    }
}

#[derive(Clone)]
pub struct IpMacSet {
    entries: Vec<IpMacCombo>,
}

impl IpMacSet {
    pub fn new() -> IpMacSet {
        IpMacSet { entries: Vec::new() }
    }

    pub fn push(&mut self, entry: IpMacCombo) {
        self.entries.push(entry)
    }

    pub fn push_ip(&mut self, entry: Ipv4Addr) {
        self.entries.push(IpMacCombo::from_ip(entry));
    }

    pub fn push_mac(&mut self, entry: MacAddress) {
        self.entries.push(IpMacCombo::from_mac(entry));
    }

    pub fn associate_ip_mac(&mut self, ip: Ipv4Addr, mac: MacAddress) -> Result<IpMacCombo, IPMACAssociateError> {
       if !self.contains_ip(ip) || !self.contains_mac(mac) {
           Err(IPMACAssociateError(ip,mac))
       }
       else {
           self.remove_by_ip(ip);
           self.remove_by_mac(mac);

           let ip_mac = IpMacCombo::new(ip,mac);
           self.push(ip_mac);
           Ok(ip_mac)
       }
    }

    pub fn get(&self, entry: usize) -> Option<&IpMacCombo> {
        self.entries.get(entry)
    }

    pub fn get_index(&self, entry: IpMacCombo) -> usize {
        let mut notfound = true;
        let mut index: usize = 0;
        if self.contains(&entry) {
            for i in 0..self.entries.len() {
                if *self.entries.get(i).unwrap() == entry {
                    index = i;
                    notfound = false;
                    break;
                }
            }
        }
        assert!(!notfound);
        index
    }

    pub fn get_indices_by_ip(&self, ip: Ipv4Addr) -> Vec<usize> {
        self.get_by_ip(ip).iter().map(|x| self.get_index(**x)).collect::<Vec<usize>>()
    }

    pub fn get_indices_by_mac(&self, mac: MacAddress) -> Vec<usize> {
        self.get_by_mac(mac).iter().map(|x| self.get_index(**x)).collect::<Vec<usize>>()
    }

    pub fn get_by_ip(&self, ip: Ipv4Addr) -> Vec<&IpMacCombo> {
        self.entries.iter().filter(|x| x.get_ip().is_some() && x.get_ip().unwrap() == ip).collect::<Vec<&IpMacCombo>>()
    }

    pub fn get_by_mac(&self, mac: MacAddress) -> Vec<&IpMacCombo> {
        self.entries.iter().filter(|x| x.get_mac().is_some() && x.get_mac().unwrap() == mac).collect::<Vec<&IpMacCombo>>()
    }

    pub fn get_by_ip_mac(&self, ip: Ipv4Addr, mac: MacAddress) -> Vec<&IpMacCombo> {
        self.entries.iter().filter(|x| x.get_ip().is_some() && x.get_mac().is_some() && x.get_ip().unwrap() == ip && x.get_mac().unwrap() == mac).collect::<Vec<&IpMacCombo>>()
    }

    pub fn contains(&self, entry: &IpMacCombo) -> bool {
        self.entries.contains(entry)
    }

    pub fn contains_ip(&self, ip: Ipv4Addr) -> bool {
        self.get_by_ip(ip).len() > 0
    }

    pub fn contains_mac(&self, mac: MacAddress) -> bool {
        self.get_by_mac(mac).len() > 0
    }

    pub fn has_multiples_of(&self, ip: Ipv4Addr, mac: MacAddress) -> bool {
        self.get_by_ip_mac(ip, mac).len() > 1
    }

    pub fn ip_has_multiple_macs(&self, ip: Ipv4Addr) -> bool {
        let ips_with_macs = self.get_by_ip(ip).iter().filter(|x| x.get_mac().is_some()).map(|x| **x).collect::<Vec<IpMacCombo>>();
        if ips_with_macs.len() < 2 {
            return false;
        }
        else {
            let first_mac = ips_with_macs.get(0).unwrap().get_mac().unwrap();
            return ips_with_macs.iter().filter(|x| x.get_mac().unwrap() != first_mac).map(|x| *x).collect::<Vec<IpMacCombo>>().len() > 0
        }
    }

    pub fn mac_has_multiple_ips(&self, mac: MacAddress) -> bool {
        let macs_with_ips = self.get_by_mac(mac).iter().filter(|x| x.get_ip().is_some()).map(|x| **x).collect::<Vec<IpMacCombo>>();
        if macs_with_ips.len() < 2 {
            return false;
        }
        else {
            let first_ip = macs_with_ips.get(0).unwrap().get_ip().unwrap();
            return macs_with_ips.iter().filter(|x| x.get_ip().unwrap() != first_ip).map(|x| *x).collect::<Vec<IpMacCombo>>().len() > 0
        }
    }

    pub fn remove(&mut self, entry: usize) -> IpMacCombo {
        self.entries.remove(entry)
    }

    pub fn remove_indices(&mut self, entries: Vec<usize>) -> Vec<IpMacCombo> {
        let mut mask: u32 = 0;
        let sorted = &mut entries.clone();
        let mut removed = Vec::new();
        for index in sorted.iter() {
            removed.push(self.remove((*index as u32 - mask) as usize));
            mask += 1;
        }
        removed
    }

    pub fn remove_by_ip(&mut self, ip: Ipv4Addr) -> Vec<IpMacCombo> {
        let indices = self.get_indices_by_ip(ip);
        self.remove_indices(indices)
    }

    pub fn remove_by_mac(&mut self, mac: MacAddress) -> Vec<IpMacCombo> {
        let indices = self.get_indices_by_mac(mac);
        self.remove_indices(indices)
    }
}
