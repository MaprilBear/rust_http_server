use proc_macro2::{Ident, Span};
use quote::quote;
use std::str::FromStr;
use strum_macros::{AsRefStr, EnumString};
#[derive(Debug, PartialEq, EnumString, AsRefStr, Default)]
pub enum HttpMethod {
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

impl PartialEq<http::Method> for HttpMethod {
    fn eq(&self, other: &http::Method) -> bool {
        match HttpMethod::from_str(other.as_str()) {
            Ok(_) => true,
            Err(_) => *self == HttpMethod::ANY,
        }
    }
}

impl PartialEq<HttpMethod> for http::Method {
    fn eq(&self, other: &HttpMethod) -> bool {
        other.eq(self)
    }
}

impl quote::ToTokens for HttpMethod {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let variant = Ident::new(self.as_ref(), Span::call_site());
        tokens.extend(quote! { ::router::HttpMethod::#variant });
    }
}
