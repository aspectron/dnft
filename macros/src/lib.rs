use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
mod schema;

#[proc_macro]
#[proc_macro_error]
pub fn build_schema(input: TokenStream) -> TokenStream {
    schema::build_schema(input)
}
