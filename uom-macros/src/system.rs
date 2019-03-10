use crate::proc_macro2::TokenStream;
use syn::parse::{Parse, ParseStream};
//use syn::{Ident, Attribute, Token};

pub struct System {
    // attrs: Vec<Attribute>,
    // name: Ident,
    // base_quantities: Vec<BaseQuantity>,
    // units: Units,
}

// struct BaseQuantity {
//     name: Ident,
//     unit: Ident,
//     symbol: Ident,
// }

// struct Units {
//     attrs: Vec<Attribute>,
//     name: Ident,
//     units: Vec<Unit>,
// }

// struct Unit {
//     add_mod: bool,
//     module: Ident,
//     quantity: Ident,
// }

impl Parse for System {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _: TokenStream = input.parse()?;

        Ok(System {})
        // let attrs = input.call(Attribute::parse_outer)?;
        // input.parse::<Ident>()?;
        // input.parse::<Token![:]>()?;
        // let name = input.parse()?;
        // let base_quantities = input.parse()?;
        // let units = input.parse()?;

        // Ok(System {
        //     attrs,
        //     name,
        //     base_quantities,
        //     units,
        // })
    }
}

// impl Parse for BaseQuantity {
//     fn parse(input: ParseStream) -> syn::Result<Self> {
//         let name = input.parse()?;
//         let unit = input.parse()?;
//         let symbol = input.parse()?;

//         Ok(BaseQuantity {
//             name,
//             unit,
//             symbol,
//         })
//     }
// }

pub fn expand(_input: System) -> Result<TokenStream, syn::Error> {
    Ok(TokenStream::new())
}
