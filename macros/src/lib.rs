extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;
use syn;

mod derive_macros;

#[proc_macro_derive(Serializeable)]
pub fn serializeable_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    derive_macros::serialiseable::impl_serializeable_macro(&ast)
}
