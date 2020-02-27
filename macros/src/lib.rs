extern crate proc_macro;
use proc_macro::TokenStream as TokenStream1;

use syn::{self, parse_macro_input, Stmt};
use syn::parse;


use quote::quote;
use quote::ToTokens;

#[proc_macro_attribute]
pub fn traced(_attr: TokenStream1, item: TokenStream1) -> TokenStream1 {
    let mut input = parse_macro_input!(item as syn::ItemFn);

    let entry: Stmt = parse(quote!{ println!("Entered autoinstrumentation"); }.into())
        .expect("Couldn't parse instrumentation entry");
    let exit: Stmt = parse(quote!{ println!("Exited autoinstrumentation"); }
        .into())
        .expect("Couldn't parse instrumentation exit");

    (*input.block).stmts.push(exit);
    (*input.block).stmts.insert(0, entry);

    let ret = input.to_token_stream();

    ret.into()
}
