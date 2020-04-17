extern crate proc_macro;
use proc_macro::TokenStream as TokenStream1;

use syn::{self, parse_macro_input,
    Stmt,
    Ident,
    FnArg,
    Pat, PatType, PatIdent,
    Path, PathSegment, PathArguments,
    punctuated::Punctuated,
    Type, TypePath,
    token,
};

use proc_macro2::{
    Span,
};

use syn::{
    parse,
    Token,
};


use quote::quote;
use quote::ToTokens;

fn prologue() -> Stmt {
    parse(quote!{
        let __lightstep_tracing_span = lightstep_tracer_context.start_span();
    }.into())
        .expect("Couldn't parse instrumentation entry")

}

fn epilogue() -> Stmt {
    parse(quote!{
        ();
        // Do we want to manually send the span or let drop deal with it?
    }.into())
        .expect("Couldn't parse instrumentation exit")
}

fn ctx_arg() -> FnArg {
    let mut ty_path = Punctuated::new();
    ty_path.push(PathSegment {
        ident: Ident::new("LightstepTracerContext", Span::call_site()),
        arguments: PathArguments::None,
    });
    FnArg::Typed(PatType {
        attrs: vec![],
        pat: Box::new(Pat::Ident(PatIdent {
            attrs: vec![],
            by_ref: None,
            mutability: None,
            ident: Ident::new("lightstep_tracer_context", Span::call_site()),
            subpat: None,
        })),
        colon_token: Token!(:)([Span::call_site()]),
        ty: Box::new(Type::Path(TypePath {
            qself: None,
            path: Path {
                leading_colon: None,
                segments: ty_path,
            },
        })),
    })
}

#[proc_macro_attribute]
pub fn traced(_attr: TokenStream1, item: TokenStream1) -> TokenStream1 {
    let mut input = parse_macro_input!(item as syn::ItemFn);

    let entry: Stmt = prologue();
    let exit: Stmt = epilogue();

    input.sig.inputs.push(ctx_arg());

    (*input.block).stmts.push(exit);
    (*input.block).stmts.insert(0, entry);

    let ret = input.to_token_stream();

    ret.into()
}

/// What should the API look like?
///
/// ```rust
/// #[traced::entry]
/// fn whatever() {
///     thing() // ::entry created a context object-
/// }
///
/// #[traced]
/// fn thing() { // And traced added an extra argument to `thing` to recieve it
///
/// }
/// ```
///
/// Does this even work? Since I need to know what other functions are traced in order to know
/// whether or not to stuff an extra argument into them?
fn _docs() {}
