//! ⚠ TODO: This crate is currently a placeholder for future development efforts. ⚠
//!
//! `uom-macros` provides procedural macro support for `uom`. Two function-style macros are
//! available. `system!`, to define a system of quantities and a related system of units, and
//! `quantity!`, to define quantities within a system. See the
//! [`uom`](https://crates.io/crates/uom) crate for full details.

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

use crate::proc_macro::TokenStream;
use crate::syn::{parse::Parse, parse_macro_input, Error};

mod quantity;
mod system;

/// TODO: Define a system of quantities.
#[proc_macro]
pub fn system(input: TokenStream) -> TokenStream {
    expand_proc_macro(input, system::expand)
}

/// TODO: Define a quantity within a system of quantities.
#[proc_macro]
pub fn quantity(input: TokenStream) -> TokenStream {
    expand_proc_macro(input, quantity::expand)
}

fn expand_proc_macro<T>(
    input: TokenStream,
    f: fn(T) -> Result<proc_macro2::TokenStream, Error>,
) -> TokenStream
where
    T: Parse,
{
    let item = parse_macro_input!(input);

    f(item).unwrap_or_else(|e| e.to_compile_error()).into()
}
