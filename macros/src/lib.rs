extern crate proc_macro;
use proc_macro::TokenStream as TokenStream1;

use syn::{self, parse_macro_input,
    Attribute, AttrStyle,
    Ident,
    Expr, ExprCall, ExprPath,
    FnArg,
    Pat, PatType, PatIdent,
    Path, PathSegment, PathArguments,
    punctuated::Punctuated,
    Stmt,
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

use lazy_static::lazy_static;

use std::sync::Mutex;
use std::collections::HashSet;
type FuncDef = ();
lazy_static! {
    static ref TRACED_FUNCS: Mutex<HashSet<FuncDef>> = Mutex::new(HashSet::new());
}

static CONTEXT_NAME: &'static str = "lightstep_tracer_context";
fn context_expr() -> ExprPath {
    let mut path = Punctuated::new();
    path.push(PathSegment {
        ident: Ident::new(CONTEXT_NAME, Span::call_site()),
        arguments: PathArguments::None,
    });
    ExprPath {
        attrs: vec![],
        qself: None,
        path: Path {
            leading_colon: None,
            segments: path,
        },

    }
}

fn is_traced(f: &ExprCall) -> bool {
    println!("{}", f.attrs.len());
    f.attrs.iter().any(|attr| {
        attr.path.segments.last()
            .map(|segment| {
                println!("Segment: {}", &segment.ident);
                segment.ident == "traced"
            })
            .unwrap_or(false)
    })
}

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
            ident: Ident::new(CONTEXT_NAME, Span::call_site()),
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

    for stmt in (*input.block).stmts.iter_mut() {
        match stmt {
            Stmt::Expr(Expr::Call(ref mut call)) |
            Stmt::Semi(Expr::Call(ref mut call), _) => {
                dbg!(is_traced(call));
                if is_traced(call) {
                    call.args.push(Expr::Path(context_expr()))
                }
            },
            _ => {},
        }
    }

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
