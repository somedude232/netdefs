#[macro_use] extern crate procedural_masquerade;
extern crate phf_codegen;
extern crate proc_macro;
#[macro_use] extern crate quote;
extern crate syn;

/* netdefs/macros/lib.rs
 *
 * Description: a disgusting proc_macro hack to enable static HashMaps in stable Rust.
 * Unfortunately, I haven't dedicated enough time to figuring out how to generalize this
 * for all types of keys. The main roadblock is using a specified key_type to cast the
 * key_values to that type, which might be easier in standard macros.
 *
 * Credit: This was modified a bit from SimonSapin's rust-cssparser proc_macro hack.
 * Kudos to him for figuring this out on stable Rust.
 *
 */

define_proc_macros! {

    // Input: parsed as token trees. The first TT is a type. (Can be wrapped in parens.)
    // following TTs are grouped in pairs, each pair being a key as a u16
    // and the corresponding value as a const expression.
    //
    // Output: a rust-phf map, with keys ASCII-lowercased:
    // ```text
    // static MAP: &'static ::netdefs::phf::Map<u16, $ValueType> = …;
    // ```
    #[allow(non_snake_case)]
    pub fn netdefs_internal__u16_key_phf_map(input: &str) -> String {
        let token_trees = syn::parse_token_trees(input).unwrap();
        let value_type = &token_trees[0];
        let pairs: Vec<_> = token_trees[1..].chunks(2).map(|chunk| {
            let key = u16_convert(&chunk[0]);
            let value = &chunk[1];
            (key, quote!(#value).to_string())
        }).collect();

        let mut map = phf_codegen::Map::new();
        map.phf_path("::netdefs::_internal__phf");
        for &(ref key, ref value) in &pairs {
            map.entry(*key, &**value);
        }

        let mut tokens = quote! {
            static MAP: ::netdefs::_internal__phf::Map<u16, #value_type> =
        };
        let mut initializer_bytes = Vec::new();
        map.build(&mut initializer_bytes).unwrap();
        tokens.append(::std::str::from_utf8(&initializer_bytes).unwrap());
        tokens.append(";");
        tokens.into_string()
    }
    
    // Exact same thing as above, but with u8 key
    #[allow(non_snake_case)]
    pub fn netdefs_internal__u8_key_phf_map(input: &str) -> String {
        let token_trees = syn::parse_token_trees(input).unwrap();
        let value_type = &token_trees[0];
        let pairs: Vec<_> = token_trees[1..].chunks(2).map(|chunk| {
            let key = u8_convert(&chunk[0]);
            let value = &chunk[1];
            (key, quote!(#value).to_string())
        }).collect();

        let mut map = phf_codegen::Map::new();
        map.phf_path("::netdefs::_internal__phf");
        for &(ref key, ref value) in &pairs {
            map.entry(*key, &**value);
        }

        let mut tokens = quote! {
            static MAP: ::netdefs::_internal__phf::Map<u8, #value_type> =
        };
        let mut initializer_bytes = Vec::new();
        map.build(&mut initializer_bytes).unwrap();
        tokens.append(::std::str::from_utf8(&initializer_bytes).unwrap());
        tokens.append(";");
        tokens.into_string()
    }
}

// Helper utility functions to convert tokens into u16/u8.
fn u16_convert(token: &syn::TokenTree) -> u16 {
    match *token {
	syn::TokenTree::Token(syn::Token::Literal(syn::Lit::Int(ref num, _))) => *num as u16,
	_ => panic!("expected u16, got {:?}", token)
    }
}

fn u8_convert(token: &syn::TokenTree) -> u8 {
    match *token {
	syn::TokenTree::Token(syn::Token::Literal(syn::Lit::Int(ref num, _))) => *num as u8,
	_ => panic!("expected u8, got {:?}", token)
    }
}
