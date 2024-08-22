use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput, Type};

#[proc_macro_derive(Resolve, attributes(response, state))]
pub fn derive_resolve(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  match impl_derive_resolve(input) {
    Ok(stream) => stream,
    Err(err) => err.into_compile_error(),
  }
  .into()
}

fn impl_derive_resolve(input: DeriveInput) -> Result<TokenStream, syn::Error> {
  let response_type = input
    .attrs
    .iter()
    .find(|attr| attr.path().is_ident("response"))
    .ok_or_else(|| syn::Error::new(input.span(), "did not find `#[response]` attribute"))?;
  let response: Type = response_type.parse_args()?;

  let state_type = input
    .attrs
    .iter()
    .find(|attr| attr.path().is_ident("state"))
    .ok_or_else(|| syn::Error::new(input.span(), "did not find `#[state]` attribute"))?;
  let state: Type = state_type.parse_args()?;

  let req = &input.ident;
  let mut res = quote! {
    impl resolver_api::HasResponse for #req {
      type State = #state;
      type Response = #response;

      fn req_type() -> &'static str {
        stringify!(#req)
      }
      fn res_type() -> &'static str {
        stringify!(#response)
      }
    }
  };

  // If the derive proc_macro target is an enum then we automatically generate
  // a `Resolve` implementation
  match input.data {
    Data::Struct(_) => {}
    Data::Enum(e) => {
      // Enforce enum variants with single unnamed field
      let idents = e
        .variants
        .into_iter()
        .map(|v| match v.fields {
          syn::Fields::Unnamed(u) => match u.unnamed.len() {
            1 => Ok(v.ident),
            _ => Err(syn::Error::new(u.unnamed.span(), "expected one enum field")),
          },
          v => Err(syn::Error::new(
            v.span(),
            "only unnamed enum fields are supported",
          )),
        })
        .collect::<Result<Vec<_>, _>>()?;

      let enum_res = quote! {
        impl ::resolver_api::Resolve for #req {
          async fn resolve(self, state: &Self::State) -> Self::Response {
            match self {
              #(#req::#idents(c) => {::core::convert::From::from(::resolver_api::Resolve::resolve(c, state).await)},)*
            }
          }
        }
      };
      res.extend(enum_res);
    }
    _ => return Err(syn::Error::new(input.span(), "unions are unsupported")),
  }
  Ok(res)
}
