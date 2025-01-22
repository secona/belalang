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
            "belalang_type attribute only supports structs with named fields"
        )
        .to_compile_error()
        .into()
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
            pub base: BelalangObject,
            #named_fields
        }

        const _: () = {
            #[crate::ctor]
            fn register() {
                crate::TYPE_REGISTRY.lock().unwrap().insert(
                    #struct_name::r#type(),
                    |obj: *const BelalangObject| -> Option<*const dyn BelalangType> {
                        Some(obj as *const #struct_name)
                    },
                );
            }
        };
    })
}

#[proc_macro_attribute]
pub fn register_belalang_type(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    let Fields::Named(fields) = &input.fields else {
        return syn::Error::new_spanned(
            input,
            "belalang_type attribute only supports structs with named fields"
        )
        .to_compile_error()
        .into()
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
            #named_fields
        }

        const _: () = {
            #[crate::ctor]
            fn register() {
                crate::TYPE_REGISTRY.lock().unwrap().insert(
                    #struct_name::r#type(),
                    |obj: *const BelalangObject| -> Option<*const dyn BelalangType> {
                        Some(obj as *const #struct_name)
                    },
                );
            }
        };
    })
}
