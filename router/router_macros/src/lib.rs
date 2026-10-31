use proc_macro::TokenStream;
use quote::quote;
use std::str::FromStr;
use strum_macros::{AsRefStr, EnumString};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    FnArg, Ident, ItemFn, LitStr, ReturnType, Signature, Token, Type, parse_macro_input,
    parse_quote,
};

#[derive(Default, Debug)]
struct RouteArgs {
    uri: String,
    method: HttpMethod,
}

struct KeyValue {
    key: Ident,
    value: LitStr,
}

impl Parse for KeyValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key = input.parse()?;
        input.parse::<Token![=]>()?;
        Ok(KeyValue { key, value: input.parse()? })
    }
}

#[derive(Debug, PartialEq, EnumString, AsRefStr, Default)]
enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    TRACE,
    PATCH,
    CONNECT,
    QUERY,

    #[default]
    ANY,
}

impl Parse for RouteArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let pairs = Punctuated::<KeyValue, Token![,]>::parse_terminated(input)?;
        let mut route_args =
            RouteArgs { uri: String::from_str("/").unwrap(), ..Default::default() };
        for kv in pairs {
            match kv.key.to_string().as_str() {
                "uri" => route_args.uri = kv.value.value(),
                "method" => match HttpMethod::from_str(kv.value.value().as_str()) {
                    Ok(method) => {
                        route_args.method = method;
                    }
                    Err(_) => {
                        return Err(syn::Error::new(kv.value.span(), "Invalid HTTP method"));
                    }
                },
                _ => return Err(syn::Error::new(kv.key.span(), "Unknown key")),
            }
        }
        Ok(route_args)
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
        Err(syn::Error::new(
            sig.span(),
            "expected signature `fn(request: Request<&[u8]>) -> Response<&[u8]>`",
        ))
    }
}

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let macro_args = parse_macro_input!(attr as RouteArgs);

    if let Err(e) = __check_signature(&func.sig) {
        return e.into_compile_error().into();
    }

    let name = func.sig.ident.to_string();
    let uri = macro_args.uri;
    let method = macro_args.method.as_ref();

    eprintln!("{name} {uri} {method}");

    quote! {
        #func

        ::router::inventory::submit! {
            ::router::RegisteredRoute {
                name: #name,
                uri: #uri,
                method: #method,
            }
        }
    }
    .into()
}
