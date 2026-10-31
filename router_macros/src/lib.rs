use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    FnArg, Ident, ItemFn, ReturnType, Signature, Token, Type, parse_macro_input, parse_quote,
};

// Define a struct to hold your parsed macro configuration
struct MacroArgs {
    pub fields: Vec<Ident>,
}

// Implement Parse to tell `syn` how to consume the tokens
impl Parse for MacroArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse a comma-separated list of literal key - string value pairs
        // Parse a comma-separated list of Idents (e.g., #[my_macro(foo = "blah", bar = "moo")])
        let vars = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
        eprintln!("{:?}", vars);
        Ok(MacroArgs { fields: vars.into_iter().collect() })
    }
}

fn __check_signature(sig: &Signature) -> syn::Result<()> {
    let expected_arg: Type = parse_quote!(Request<&[u8]>);
    let expected_ret: Type = parse_quote!(Response<Vec<u8>>);

    let arg_ok = match sig.inputs.iter().collect::<Vec<_>>().as_slice() {
        [FnArg::Typed(arg)] => *arg.ty == expected_arg,
        _ => false,
    };
    let ret_ok = matches!(&sig.output, ReturnType::Type(_, ty) if **ty == expected_ret);

    if arg_ok && ret_ok {
        Ok(())
    } else {
        Err(syn::Error::new(sig.span(), "expected function signature"))
    }
}

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let name = func.sig.ident.clone().to_string();
    let macro_args = parse_macro_input!(attr as MacroArgs);

    let uri = match macro_args.fields.len() {
        1 => macro_args.fields[0].to_string(),
        _ => "/".to_string(),
    };

    if __check_signature(&func.sig).is_err() {
        panic!("expected signature `fn(request: Request<&[u8]>) -> Response<&[u8]>`");
    }

    let emitted_output = quote! {
        ::router::inventory::submit! {
            ::router::RegisteredRoute {
                name: #name,
                uri: #uri,
            }
        }

        #func
    };

    eprintln!("{}", emitted_output);

    emitted_output.into()
}
