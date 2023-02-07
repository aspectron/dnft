use crate::handler::*;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::convert::Into;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    Error, Expr, ExprArray, Result, Token,
};

#[derive(Debug)]
struct SchemaField {
    // schema_type: Expr,
    handlers: ExprArray,
}

impl Parse for SchemaField {
    fn parse(input: ParseStream) -> Result<Self> {
        let parsed = Punctuated::<Expr, Token![,]>::parse_terminated(input).unwrap();
        if parsed.len() != 1 {
            return Err(Error::new_spanned(
                parsed,
                "usage: build_wrpc_client_interface!(interface, RpcApiOps,[getInfo, ..])".to_string(),
            ));
        }

        let mut iter = parsed.iter();
        // Intake the enum name
        let rpc_api_ops = iter.next().unwrap().clone();
        // Intake enum variants as an array
        let handlers = get_handlers(iter.next().unwrap().clone())?;

        Ok(SchemaField { rpc_api_ops, handlers })
    }
}

impl ToTokens for SchemaField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut targets = Vec::new();
        let rpc_api_ops = &self.rpc_api_ops;

        for handler in self.handlers.elems.iter() {
            let name = handler.to_token_stream().to_string();
            let fn_subscribe_snake = Ident::new(&subscribe.to_case(Case::Snake), Span::call_site());

            // let Handler { fn_call, request_type, response_type, .. } = Handler::new(handler);

            // async fn #fn_call(&self, request : #request_type) -> RpcResult<#response_type> {
            //     let response: ClientResult<#response_type> = self.rpc.call(#rpc_api_ops::#handler, request).await;
            //     Ok(response.map_err(|e| e.to_string())?)
            // }

            // Due to conflicts between #[async_trait] macro and other macros,
            // the async implementation of the RPC caller is inlined
            targets.push(quote! {

                fn #fn_call<'life0, 'async_trait>(
                    &'life0 self,
                    request: #request_type,
                ) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = RpcResult<#response_type>> + ::core::marker::Send + 'async_trait>>
                where
                    'life0: 'async_trait,
                    Self: 'async_trait,
                {
                    Box::pin(async move {
                        if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<RpcResult<#response_type>> {
                            return __ret;
                        }
                        let __self = self;
                        //let request = request;
                        let __ret: RpcResult<#response_type> = {
                            let resp: ClientResult<#response_type> = __self.rpc.call(#rpc_api_ops::#handler, request).await;
                            Ok(resp.map_err(|e| e.to_string())?)
                        };
                        #[allow(unreachable_code)]
                        __ret
                    })
                }

            });
        }

        quote! {
            #(#targets)*
        }
        .to_tokens(tokens);
    }
}

pub fn build_wrpc_client_interface(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let rpc_table = parse_macro_input!(input as SchemaField);
    let ts = rpc_table.to_token_stream();
    // println!("MACRO: {}", ts.to_string());
    ts.into()
}
