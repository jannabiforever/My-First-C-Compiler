//! Procedural macros for the compiler project.
//!
//! Currently provides:
//!
//! - `#[derive(Statement)]`
//!   Implements `From<ThisStruct<T>> for Statement<T>` where
//!   the struct is assumed to correspond to a variant of the
//!   `Statement` enum with the same name.
//!
//!   Example:
//!   ```ignore
//!   #[derive(Debug, Clone, Statement)]
//!   pub struct BlockStmt<'a> {
//!       pub statements: Vec<Statement<'a>>,
//!   }
//!
//!   #[derive(Debug, Clone)]
//!   pub enum Statement<'a> {
//!       BlockStmt(BlockStmt<'a>),
//!       // ...
//!   }
//!   ```
//!
//!   The derive macro will generate:
//!   ```ignore
//!   impl<'a> From<BlockStmt<'a>> for Statement<'a> {
//!       fn from(value: BlockStmt<'a>) -> Self {
//!           Statement::BlockStmt(value)
//!       }
//!   }
//!   ```

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Ident, parse_macro_input};

/// Derive macro for automatically implementing `From<S>` for `Statement`,
/// where `S` is the struct the macro is applied to.
///
/// Assumptions / conventions:
/// - There exists an enum named `Statement` in scope.
/// - The enum has a variant whose name is exactly the same as the struct
///   name the macro is applied to.
/// - The generic parameters (lifetimes / type params) match between the
///   struct and the enum, e.g.:
///
///   ```ignore
///   #[derive(Debug, Clone, Statement)]
///   pub struct IfStmt<'a> {
///       pub cond: Expression<'a>,
///       pub then_block: Box<Statement<'a>>,
///       pub else_block: Option<Box<Statement<'a>>>,
///   }
///
///   #[derive(Debug, Clone)]
///   pub enum Statement<'a> {
///       IfStmt(IfStmt<'a>),
///       // ...
///   }
///   ```
///
/// What gets generated:
///
/// ```ignore
/// impl<'a> From<IfStmt<'a>> for Statement<'a> {
///     fn from(value: IfStmt<'a>) -> Self {
///         Statement::IfStmt(value)
///     }
/// }
/// ```
///
/// Limitations:
/// - This derive macro does NOT create or modify the `Statement` enum.
///   You still have to declare the enum and its variants yourself.
/// - The enum name is hard-coded as `Statement`. If you need a different
///   enum name or path (e.g. `ast::Statement`), a separate attribute-based
///   macro would be required.
#[proc_macro_derive(Statement)]
pub fn derive_statement(input: TokenStream) -> TokenStream {
    // Parse the input as a derive input (struct / enum / union).
    let input = parse_macro_input!(input as DeriveInput);

    let struct_ident: Ident = input.ident;

    // Only support structs. If someone tries to use this on an enum or union,
    // emit a compile error.
    match input.data {
        Data::Struct(_) => {}
        _ => {
            return syn::Error::new_spanned(
                struct_ident,
                "#[derive(Statement)] is only supported on structs",
            )
            .to_compile_error()
            .into();
        }
    }

    // Use the struct's generics for the impl and for the Statement type.
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // The enum we target is named `Statement`.
    let enum_ident = Ident::new("Statement", struct_ident.span());

    // Generate:
    //
    // impl <impl_generics> From<Struct<ty_generics>> for Statement<ty_generics> where_clause {
    //     fn from(value: Struct<ty_generics>) -> Self {
    //         Statement::Struct(value)
    //     }
    // }
    let expanded = quote! {
        impl #impl_generics From<#struct_ident #ty_generics> for #enum_ident #ty_generics #where_clause {
            fn from(value: #struct_ident #ty_generics) -> Self {
                #enum_ident::#struct_ident(value)
            }
        }
    };

    TokenStream::from(expanded)
}
