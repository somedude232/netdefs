// Used for private function testing only. No definitions here.

#![crate_name = "netdefs"]

#[macro_use] extern crate netdefs_macros;
#[macro_use] extern crate procedural_masquerade;
#[doc(hidden)] pub extern crate phf as _internal__phf;
extern crate regex;

pub use netdefs_macros::*;

mod netdefs { pub use _internal__phf; }

#[macro_use]
mod macros;

pub mod layer2;
pub mod layer3;
pub mod layer4;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
