mod types;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};


#[proc_macro_attribute]
pub fn handler(_attr: TokenStream, item: TokenStream) -> TokenStream {
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
