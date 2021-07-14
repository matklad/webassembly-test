#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn webassembly_test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(item as syn::ItemFn);
    let name = item.sig.ident.to_string();

    let mut ignore = "";
    if let Some(i) = item.attrs.iter().position(|attr| is_ignore(attr)) {
        item.attrs.remove(i);
        ignore = "ignore$"
    }

    let res = quote! {
        #[cfg(test)]
        #[export_name = concat!("$webassembly-test$", #ignore, module_path!(), "::",  #name)]
        #item
    };
    // eprintln!("{}", res);
    res.into()
}

fn is_ignore(attr: &syn::Attribute) -> bool {
    attr.path.is_ident("ignore")
}
