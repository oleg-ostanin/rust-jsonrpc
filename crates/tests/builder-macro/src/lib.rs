use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(HelloBuilder)]
pub fn builder_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    crate::impl_hello_builder(&ast)
}

fn impl_hello_builder(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloBuilder for #name {
            fn hello_builder() {
                println!("Hello, Builder! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}