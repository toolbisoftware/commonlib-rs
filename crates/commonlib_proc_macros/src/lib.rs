// Copyright (c) Toolbi Software. All rights reserved.
// Check the README file in the project root for more information.

use commonlib_macros::as_variant;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, Expr, Lit, Token};

enum InputKind {
  Expr(Expr),
  Lit(Lit),
}

#[proc_macro]
pub fn error(input: TokenStream) -> TokenStream {
  let args = parse_macro_input!(input with Punctuated::<InputKind, Token![,]>::parse_terminated);

  let message = (args.len() > 0)
    .then(|| &args[0])
    .map(|a| as_variant!(a, InputKind::Lit).unwrap())
    .unwrap();

  let category = (args.len() > 1)
    .then(|| &args[1])
    .map(|a| as_variant!(a, InputKind::Lit).unwrap())
    .map(|a| quote! { Some(#a) })
    .unwrap_or(quote! { None });

  let source = (args.len() > 2)
    .then(|| &args[2])
    .map(|a| as_variant!(a, InputKind::Expr).unwrap())
    .map(|a| {
      quote! {
        {
          error = error.set_source(#a);
        }
      }
    })
    .unwrap_or(quote! {});

  quote! {
    {
      let message = #message;
      let category = #category;

      let mut error = commonlib::Error::new(message);

      if let Some(category) = category {
        error = error.set_category(category);
      }

      #source

      error
    }
  }
  .into()
}

#[proc_macro]
pub fn errorf(input: TokenStream) -> TokenStream {
  let args = parse_macro_input!(input with Punctuated::<InputKind, Token![,]>::parse_terminated);

  let error = (args.len() > 0)
    .then(|| &args[0])
    .map(|a| as_variant!(a, InputKind::Expr).unwrap())
    .unwrap();

  let category = (args.len() > 1)
    .then(|| &args[1])
    .map(|a| as_variant!(a, InputKind::Lit).unwrap())
    .map(|a| quote! { Some(#a) })
    .unwrap_or(quote! { None });

  quote! {
    {
      let category = #category;

      let mut error = commonlib::Error::from_error(#error);

      if let Some(category) = category {
        error = error.set_category(category);
      }

      error
    }
  }
  .into()
}

impl syn::parse::Parse for InputKind {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    if input.peek(Lit) {
      Ok(InputKind::Lit(input.parse()?))
    } else {
      Ok(InputKind::Expr(input.parse()?))
    }
  }
}

impl quote::ToTokens for InputKind {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    match self {
      InputKind::Expr(expr) => expr.to_tokens(tokens),
      InputKind::Lit(lit) => lit.to_tokens(tokens),
    }
  }
}
