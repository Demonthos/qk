mod component;
mod component_visitor;
mod component_visitor_mut;
mod dynamic;
mod format;
mod memo;
mod node;
mod prop;
mod read_builder;
mod rsx;
mod state;

use component::Component;
use proc_macro::TokenStream;
use quote::quote;
use rsx::Elements;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn component(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Component);

    TokenStream::from(quote! {
        #input
    })
}

#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Elements);

    TokenStream::from(quote! {
        #input
    })
}
