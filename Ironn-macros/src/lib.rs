mod types;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};


#[proc_macro_attribute]
pub fn ironn_handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let vis = &input.vis;
    let name = &input.sig.ident;
    let block = &input.block;

    let expanded = quote! {
        #vis fn #name() -> std::sync::Arc<dyn Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = actix_web::HttpResponse> + Send>> + Send + Sync> {
            std::sync::Arc::new(|| {
                let fut = async move #block;
                std::pin::Pin::from(Box::new(fut))
            })
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn protected(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as syn::AttributeArgs);
    let input = parse_macro_input!(item as ItemFn);
    let vis = &input.vis;
    let sig = &input.sig;
    let name = &input.sig.ident;
    let block = &input.block;

    let mut required_role = None;
    for arg in args{
        if let syn::NestedMeta::Meta(syn::Meta::NameValue(nv)) = arg {
            if nv.path.is_ident("role"){
                if let syn::Lit::Str(s) = &nv.lit {
                    required_role = Some(s.value());
                }
            }
        }
    }
    let role_check = if let Some(role) = required_role {
        quote! {
            if !check_user_role(#role) {
                return Err(unauthorized_response());
            }
        }
    } else {
        quote! {} // no role check if none specified
    };

    let expanded = quote! {
        #vis #sig {
            #role_check
            #block
        }
    };

    TokenStream::from(expanded)
}