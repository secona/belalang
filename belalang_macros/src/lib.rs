use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Fields, ItemStruct};

// TODO: add documentation and clear namings

#[proc_macro_attribute]
pub fn belalang_type(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    let Fields::Named(fields) = &input.fields else {
        return syn::Error::new_spanned(
            input,
            "belalang_type attribute only supports structs with named fields",
        )
        .to_compile_error()
        .into();
    };

    let struct_name = &input.ident;
    let attrs = &input.attrs;
    let vis = &input.vis;
    let generics = &input.generics;
    let named_fields = &fields.named;

    TokenStream::from(quote! {
        #(#attrs)*
        #[repr(C)]
        #[derive(Debug)]
        #vis struct #struct_name #generics {
            pub base: crate::BelalangBase,
            #named_fields
        }
    })
}
