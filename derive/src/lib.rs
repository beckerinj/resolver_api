use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Type};

#[proc_macro_derive(
  Resolver,
  attributes(resolver_target, resolver_error, to_string_resolver, resolver_args)
)]
pub fn derive_resolver(input: TokenStream) -> TokenStream {
  let DeriveInput {
    ident, data, attrs, ..
  } = parse_macro_input!(input as DeriveInput);

  let (std_variant, to_string_variant, req_types) = if let Data::Enum(e) = data {
    let mut std_variants = Vec::new();
    let mut to_string_variants = Vec::new();
    let mut req_types = Vec::new();
    for v in e.variants {
      let v_ident = v.ident;
      req_types.push(quote!(#ident::#v_ident(_) => stringify!(#v_ident)));
      let use_to_string = v
        .attrs
        .iter()
        .any(|a| a.path().is_ident("to_string_resolver"));
      if use_to_string {
        to_string_variants.push(v_ident);
      } else {
        std_variants.push(v_ident);
      }
    }
    (std_variants, to_string_variants, req_types)
  } else {
    panic!("expected request enum")
  };

  let target_attr = attrs
    .iter()
    .find(|attr| attr.path().is_ident("resolver_target"))
    .expect("did not find resolver_target attribute");

  let target: Type = target_attr
    .parse_args()
    .expect("should pass struct to implement resolve_request on, eg. AppState");

  let error = attrs
    .iter()
    .find(|attr| attr.path().is_ident("resolver_error"))
    .and_then(|attr| attr.parse_args().ok())
    .unwrap_or(quote!(anyhow::Error));

  let args_attr = attrs
    .iter()
    .find(|attr| attr.path().is_ident("resolver_args"));

  let args: proc_macro2::TokenStream = match args_attr {
    Some(args) => args
      .parse_args()
      .expect("should pass args type to resolver_args attr, or remove to default to ()"),
    None => quote!(()),
  };

  quote! {
    impl resolver_api::Resolver<#ident, #args, #error> for #target {
      async fn resolve_request(&self, request: #ident, args: #args)
        -> Result<String, resolver_api::Error<#error>>
      {
        match request {
          #(#ident::#std_variant(req) => 
            <#target as resolver_api::Resolve<_, _, _>>::resolve_response(self, req, args)
              .await,)*
          #(#ident::#to_string_variant(req) => 
            <#target as resolver_api::ResolveToString<_, _, _>>::resolve_to_string(self, req, args)
              .await
              .map_err(resolver_api::Error::Inner),)*
        }
      }
    }
    impl #ident {
      pub fn req_type(&self) -> &'static str {
        match self {
          #(#req_types,)*
        }
      }
    }
  }
  .into()
}

#[proc_macro_derive(Request, attributes(response))]
pub fn has_response(input: TokenStream) -> TokenStream {
  let input = parse_macro_input!(input as DeriveInput);
  let req = input.ident;

  let attr = input
    .attrs
    .into_iter()
    .find(|attr| attr.path().is_ident("response"))
    .expect("did not find response attribute");

  let res: Type = attr.parse_args().expect("should pass response type");

  quote! {
    impl resolver_api::HasResponse for #req {
      type Response = #res;
      fn req_type() -> &'static str {
        stringify!(#req)
      }
      fn res_type() -> &'static str {
        stringify!(#res)
      }
    }
  }
  .into()
}
