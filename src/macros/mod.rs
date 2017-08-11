define_invoke_proc_macro!(netdefs_internal__invoke_proc_macro);

/* src/macros/mod.rs
 *
 * Description: The callable macro definitions for the proc_macros defined
 * in macros/lib.rs. These will be what you call if you want to create a
 * static HashMap. See an example of this call in src/layer2/ethernet.rs
 *
 * Credit: This was modified a bit from SimonSapin's rust-cssparser proc_macro hack.
 * Kudos to him for figuring this out on stable Rust.
 *
 */

#[macro_export]
macro_rules! u16_key_phf_map {
    ($name: ident -> $ValueType: ty = { $( $key: expr => $value: expr ),* }) => {
        u16_key_phf_map!($name -> $ValueType = { $( $key => $value, )* })  // This calls the line below
    };
    ($name: ident -> $ValueType: ty = { $( $key: expr => $value: expr, )* }) => {
        #[allow(non_snake_case)]
        pub fn $name(input: u16) -> Option<&'static $ValueType> {
            netdefs_internal__invoke_proc_macro! {
		netdefs_internal__u16_key_phf_map!( ($ValueType) $( $key ($value) )+ )
            }
            {
                MAP.get(&input)
            }
        }
    }
}

#[macro_export]
macro_rules! u8_key_phf_map {
    ($name: ident -> $ValueType: ty = { $( $key: expr => $value: expr ),* }) => {
        u8_key_phf_map!($name -> $ValueType = { $( $key => $value, )* })  // This calls the line below
    };
    ($name: ident -> $ValueType: ty = { $( $key: expr => $value: expr, )* }) => {
        #[allow(non_snake_case)]
        pub fn $name(input: u8) -> Option<&'static $ValueType> {
            netdefs_internal__invoke_proc_macro! {
		netdefs_internal__u8_key_phf_map!( ($ValueType) $( $key ($value) )+ )
            }
            {
                MAP.get(&input)
            }
        }
    }
}
