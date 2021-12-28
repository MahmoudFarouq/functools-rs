extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, parse_quote, Pat, Type};

#[proc_macro_attribute]
pub fn cache(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut input_fn = parse_macro_input!(item as syn::ItemFn);

    let fn_name = input_fn.sig.ident.clone();
    let fn_arguments = input_fn.sig.inputs.clone();
    let fn_output = input_fn.sig.output.clone();

    let dada = input_fn.block.stmts.clone();

    let cache_value_type = if let syn::ReturnType::Type(_, t) = fn_output.clone() {
        t
    } else {
        panic!("cannot find output type for {}", fn_name)
    };

    let cache_key_type = fn_arguments.iter().map(|arg| {
        if let syn::FnArg::Typed(x) = arg {
            x.ty.clone()
        } else {
            panic!("Oof")
        }
    });
    let cache_key_type: Punctuated<Box<Type>, Comma> = Punctuated::from_iter(cache_key_type);

    let parameters_names = fn_arguments.iter().map(|arg| {
        if let syn::FnArg::Typed(x) = arg {
            x.pat.clone()
        } else {
            panic!("Oof")
        }
    });
    let parameters_names: Punctuated<Box<Pat>, Comma> = Punctuated::from_iter(parameters_names);

    input_fn.block = Box::new(parse_quote! {{
        fn __inner(#fn_arguments, memo: &mut HashMap<(#cache_key_type), #cache_value_type>) #fn_output {
            if let Some(r) = memo.get(&(#parameters_names)) {
                return r.clone();
            }

            let mut #fn_name = |#parameters_names| __inner(#parameters_names, memo);

            let mut r = || {#(#dada)*};
            let r = r();

            memo.insert((#parameters_names), r);

            r
        }

        let mut memo = HashMap::new();
        __inner(#parameters_names, &mut memo)
    }});

    TokenStream::from(quote! {#input_fn})
}
