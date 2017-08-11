# netdefs-rs

netdefs-rs is a library that provides definitions and structures for common networking
protocols and formats (e.g. IPv4, IPv6, MAC addresses, etc). It also provides utilities
like converting raw field values into enumerated types for readability in code (EtherType in
Ethernet headers, Protocol field in IPv4 headers)

**NOTE**: This library is in very early development. You probably shouldn't use this in production.

Bug reports, bug fixes, feature requests, and feature additions are welcome.

## Requirements
### Language Version
* Tested on Rust 1.18.0 on x86_64-unknown-linux-gnu (CentOS 7)

### Crates
* `phf = 0.7.21`
* `procedural-masquerade = 0.1` (bundled in /procedural-masquerade/)
* `regex = 0.2.2`
* `chrono = 0.4`
* `syn = { version = "0.11", features = ["full"] }`
* `quote = 0.3`

## Installing from Cargo
* Coming soon! (Once the crate is published on crates.io)

## Installing from Source
### Cloning the Repo
* `cd ~/my-git-or-rust-workspace/`
* `git clone https://github.com/somedude232/netdefs/`

## Usage
Specify the crate in your `Cargo.toml`: `netdefs = "0.1"`

And make sure to import it at the top of your Rust files: `extern crate netdefs;`

Full code documentation coming soon.

The library is divided into layers and protocols. So, for example, to import structures and functions to handle Ethernet things: `use netdefs::layer2::ethernet::{ThingYouWantToImport,OtherThing};`

A basic example:
```
extern crate netdefs;

use netdefs::layer2::{parse_ethertype}
use netdefs::layer3::NetworkProtocol;

fn main() {
    let unknown_ethertype: u16 = 0x0800;
    println!("This EtherType is the {} Layer 3 Protocol.", parse_ethertype(unknown_ethertype).unwrap());
  
    let mac = MacAddress::from_str("12:34:56:78:90:AB");
    let mac_alt = MacAddress::from_str("12-34-56-78-90-AB");
    assert_eq!(mac,mac_alt);
}

```
