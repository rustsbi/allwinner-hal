//! Attribute macro for FEL payload entry functions.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    FnArg, ItemFn, PointerMutability, ReturnType, Type, Visibility, parse_macro_input,
    spanned::Spanned,
};

/// Export a private `fn(parameters: *mut u32)` as the unsafe C-ABI FEL entry.
/// Use `#[entry(align_stack)]` when the ROM stack needs ABI alignment.
#[proc_macro_attribute]
pub fn entry(args: TokenStream, input: TokenStream) -> TokenStream {
    let function = parse_macro_input!(input as ItemFn);
    expand_entry(args.into(), function)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn expand_entry(args: TokenStream2, function: ItemFn) -> syn::Result<TokenStream2> {
    let align_stack = args.to_string() == "align_stack";
    if !args.is_empty() && !align_stack {
        return Err(syn::Error::new_spanned(
            args,
            "expected `#[entry]` or `#[entry(align_stack)]`",
        ));
    }

    let signature = &function.sig;
    let returns_unit = match &signature.output {
        ReturnType::Default => true,
        ReturnType::Type(_, ty) => matches!(&**ty, Type::Tuple(tuple) if tuple.elems.is_empty()),
    };
    if !matches!(function.vis, Visibility::Inherited)
        || signature.constness.is_some()
        || signature.asyncness.is_some()
        || signature.abi.is_some()
        || !signature.generics.params.is_empty()
        || signature.generics.where_clause.is_some()
        || signature.variadic.is_some()
        || !returns_unit
        || signature.inputs.len() != 1
    {
        return Err(syn::Error::new(
            signature.span(),
            "`#[entry]` requires a private, non-generic `fn(parameters: *mut u32)` returning `()`",
        ));
    }
    let parameter = signature.inputs.first().unwrap();
    let valid_parameter = match parameter {
        FnArg::Typed(argument) => match &*argument.ty {
            Type::Ptr(pointer) if matches!(pointer.mutability, PointerMutability::Mut(_)) => {
                matches!(&*pointer.elem, Type::Path(path) if path.qself.is_none() && path.path.is_ident("u32"))
            }
            _ => false,
        },
        FnArg::Receiver(_) => false,
    };
    if !valid_parameter {
        return Err(syn::Error::new_spanned(
            parameter,
            "`#[entry]` parameter must have type `*mut u32`",
        ));
    }

    let attrs = &function.attrs;
    let name = &signature.ident;
    let inputs = &signature.inputs;
    let statements = &function.block.stmts;
    if align_stack {
        return Ok(quote! {
            #(#attrs)*
            unsafe extern "C" fn #name(#inputs) {
                use ::rfel_payload as _;
                #(#statements)*
            }

            #[cfg(all(target_os = "none", any(target_arch = "riscv32", target_arch = "riscv64")))]
            #[unsafe(export_name = "__rfel_payload__main")]
            #[unsafe(naked)]
            unsafe extern "C" fn __rfel_payload_aligned_entry(_parameters: *mut u32) {
                core::arch::naked_asm!(
                    ".option push",
                    ".option norelax",
                    "mv t0, sp",
                    "andi sp, sp, -16",
                    "addi sp, sp, -16",
                    ".if {rv64}",
                    "sd t0, 0(sp)",
                    "sd ra, 8(sp)",
                    ".else",
                    "sw t0, 0(sp)",
                    "sw ra, 4(sp)",
                    ".endif",
                    "call {body}",
                    ".if {rv64}",
                    "ld t0, 0(sp)",
                    "ld ra, 8(sp)",
                    ".else",
                    "lw t0, 0(sp)",
                    "lw ra, 4(sp)",
                    ".endif",
                    "mv sp, t0",
                    "ret",
                    ".option pop",
                    rv64 = const cfg!(target_arch = "riscv64") as usize,
                    body = sym #name,
                );
            }
        });
    }
    Ok(quote! {
        #(#attrs)*
        #[unsafe(export_name = "__rfel_payload__main")]
        unsafe extern "C" fn #name(#inputs) {
            // Link the runtime even when it is otherwise used only for this macro.
            use ::rfel_payload as _;
            #(#statements)*
        }
    })
}

#[cfg(test)]
mod tests {
    use super::expand_entry;
    use quote::quote;
    use syn::parse_quote;

    #[test]
    fn rejects_attribute_arguments() {
        let function = parse_quote!(
            fn payload(parameters: *mut u32) {}
        );
        assert!(expand_entry(quote!(unexpected), function).is_err());
    }

    #[test]
    fn accepts_aligned_entry() {
        let function = parse_quote!(
            fn payload(parameters: *mut u32) {}
        );
        let expansion = expand_entry(quote!(align_stack), function).unwrap();
        let file: syn::File = syn::parse2(expansion).unwrap();
        assert_eq!(file.items.len(), 2);
    }

    #[test]
    fn rejects_incompatible_entry_signatures() {
        for input in [
            "fn payload() {}",
            "fn payload(a: *mut u32, b: *mut u32) {}",
            "fn payload(parameters: *const u32) {}",
            "fn payload(parameters: *mut u64) {}",
            "fn payload(parameters: &mut u32) {}",
            "fn payload(&self) {}",
            "pub fn payload(parameters: *mut u32) {}",
            "const fn payload(parameters: *mut u32) {}",
            "async fn payload(parameters: *mut u32) {}",
            "extern \"C\" fn payload(parameters: *mut u32) {}",
            "fn payload<T>(parameters: *mut u32) {}",
            "fn payload(parameters: *mut u32) where u32: Copy {}",
            "fn payload(parameters: *mut u32) -> u32 { 0 }",
            "fn payload(parameters: *mut u32) -> ! { loop {} }",
        ] {
            let function = syn::parse_str(input).unwrap();
            assert!(expand_entry(quote!(), function).is_err(), "{input}");
        }
    }
}
