//! D1 ROM runtime procedural macros.

use proc_macro2::Span;
use quote::quote;
use syn::{
    parse, parse_macro_input, spanned::Spanned, FnArg, ItemFn, ReturnType, Type, Visibility,
};

use proc_macro::TokenStream;

/// ROM stage function entry.
#[proc_macro_attribute]
pub fn entry(args: TokenStream, input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as ItemFn);

    // check the function arguments
    if f.sig.inputs.len() != 2 {
        return parse::Error::new(
            f.sig.inputs.last().unwrap().span(),
            "`#[entry]` function should include exactly two parameters",
        )
        .to_compile_error()
        .into();
    }

    for arg in &f.sig.inputs {
        match arg {
            FnArg::Receiver(_) => {
                return parse::Error::new(arg.span(), "invalid argument")
                    .to_compile_error()
                    .into();
            }
            FnArg::Typed(t) => {
                if let Type::Path(_p) = &*t.ty {
                    // empty
                } else {
                    return parse::Error::new(t.ty.span(), "argument type must be a path")
                        .to_compile_error()
                        .into();
                }
            }
        }
    }

    // check the function signature
    let valid_signature = f.sig.constness.is_none()
        && f.sig.asyncness.is_none()
        && f.vis == Visibility::Inherited
        && f.sig.abi.is_none()
        && f.sig.generics.params.is_empty()
        && f.sig.generics.where_clause.is_none()
        && f.sig.variadic.is_none()
        && match f.sig.output {
            ReturnType::Default => true,
            ReturnType::Type(_, ref ty) => {
                matches!(&**ty, Type::Never(_))
            }
        };

    if !valid_signature {
        return parse::Error::new(
            f.span(),
            "`#[entry]` function must have signature `[unsafe] fn(p: Peripherals, c: Clocks) -> !`",
        )
        .to_compile_error()
        .into();
    }

    if !args.is_empty() {
        return parse::Error::new(Span::call_site(), "This attribute accepts no arguments")
            .to_compile_error()
            .into();
    }

    let attrs = f.attrs;
    let unsafety = f.sig.unsafety;
    let args = f.sig.inputs;
    let stmts = f.block.stmts;
    let ret = f.sig.output;

    quote!(
        #[export_name = "main"]
        pub fn main() -> ! {
            let p = unsafe { core::mem::transmute(()) };
            let c = ::allwinner_rt::__rom_init_clocks();
            unsafe { __allwinner_rt_macros__main(p, c) }
        }
        #[allow(non_snake_case)]
        #(#attrs)*
        #unsafety fn __allwinner_rt_macros__main(#args) #ret {
            #(#stmts)*
        }
    )
    .into()
}
