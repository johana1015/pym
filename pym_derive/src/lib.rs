extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Pym)]
pub fn pym_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    println!("{:#?}", &ast);

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let gen = quote! {
        impl #impl_generics Pym<String, bool, String> for #name #ty_generics #where_clause {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }

            fn exec(&mut self, args: String) -> Result<bool, String> {
                Ok(true)
            }
        }
    };
    gen.into()
}
