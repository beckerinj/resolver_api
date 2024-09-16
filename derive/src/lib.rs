use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Attribute, Data, DeriveInput, Type};

#[proc_macro_derive(Resolve, attributes(response, args))]
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
  let response_type: Type = response_type.parse_args()?;

  let ident = &input.ident;
  let mut res = quote! {
    impl resolver_api::HasResponse for #ident {
      type Response = #response_type;

      fn req_type() -> &'static str {
        stringify!(#ident)
      }
      fn res_type() -> &'static str {
        stringify!(#response_type)
      }
    }
  };

  // If the derive proc_macro target is an enum then we automatically generate
  // a `Resolve` implementation
  match input.data {
    Data::Struct(_) => {}
    Data::Enum(e) => {
      let args_type = extract_type_from_attr("args", &input.attrs)?;

      // Enforce enum variants with single unnamed field
      let variants = e
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
        impl ::resolver_api::Resolve<#args_type> for #ident {
          async fn resolve(self, args: &#args_type) -> Self::Response {
            match self {
              #(#ident::#variants(request) => {
                ::core::convert::From::from(::resolver_api::Resolve::resolve(request, args).await)
              },)*
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

fn extract_type_from_attr(ident: &str, attrs: &[Attribute]) -> Result<Type, syn::Error> {
  let res = attrs
    .iter()
    .find(|attr| attr.path().is_ident(ident))
    .map(|ty| ty.parse_args::<Type>())
    .transpose()?
    .unwrap_or_else(|| syn::parse_quote!(()));
  Ok(res)
}
