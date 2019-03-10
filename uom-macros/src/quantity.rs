use crate::proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};

pub struct Quantity;

impl Parse for Quantity {
    fn parse(_input: ParseStream) -> syn::Result<Self> {
        let _: TokenStream = _input.parse()?;

        Ok(Quantity)
    }
}

pub fn expand(_input: Quantity) -> Result<TokenStream, syn::Error> {
    Ok(TokenStream::new())
}
