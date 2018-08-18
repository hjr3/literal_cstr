extern crate proc_macro;
#[macro_use]
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::Expr::Lit;
use syn::Lit::Str;

#[proc_macro]
pub fn c(input: TokenStream) -> TokenStream {
    let ast: syn::Expr = syn::parse(input).unwrap();

    let s = match ast {
        Lit(expr_lit) => {
            match expr_lit.lit {
                Str(s) => {
                    s.value()
                }
                _ => {
                    panic!("Input must be a literal string");
                }
            }
        }
        _ => {
            panic!("Input must be a literal string");
        }
    };

    let expanded = quote!(
        ::std::ffi::CString::new(#s).unwrap()
    );

    expanded.into()
}
