extern crate proc_macro;

use proc_macro::TokenStream;
use syn;

mod derive_macros;

#[proc_macro_derive(Serializable)]
pub fn serializable_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    derive_macros::serialiseable::impl_serializable_macro(&ast)
}
