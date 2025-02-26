//! Crate to define proc-macros
//! Procedural macros also intervene on the AST compilation stage
//! Procedural macros have to be defined in their own crate
//! For a compiler a proc macro is just a function that takes a `TokenStream` and returns another
//! `TokenStream`
//! To help manipulating these `TokenStream` there exist crates designed for proc macros
//! - [syn] parses `TokenStream` into Rust AST
//! - [quote] convert Rust code into a `TokenStream` (used for the proc macro output)
//! - [proc-macro2] wrapper allowing devs to use internal types outside of proc macros context.
//!   For example it allows both _syn_ and _quote_ types to be used in regular Rust code as well
//! - [darling] helper functions to facilitate working with macro arguments
//!
//!
//! The goal here is to implement the three types of proc macros for logging

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

/// Macro that adds [INFO] at the beginning and replace any instance of the
/// word TIME with a timestamp
#[proc_macro]
pub fn function_macro_log(input: TokenStream) -> TokenStream {
    let mut output = String::from("[function proc macro] ");
    for token in input {
        let token_string = token.to_string();
        for word in token_string.split(" ") {
            output.push(' ');
            if word == "TIME" {
                let timestamp = chrono::Utc::now().time().to_string();
                output.push_str(&timestamp);
            } else {
                output.push_str(word);
            }
        }
    }
    quote! { println!(#output); }.into()
}

/// Derive macro that implements the `Log` trait defined above
/// If a field has the attribute `toto` then adds an additional message
/// Inspiration from serde code:  https://github.com/serde-rs/serde/blob/master/serde_derive/src/internals/attr.rs
#[proc_macro_derive(Log, attributes(toto))]
pub fn derive_log(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).expect("Not parsing a struct/enum/union");
    let type_name = ast.ident;

    let mut toto_fields = "".to_string();
    if let syn::Data::Struct(data) = ast.data {
        for field in data.fields {
            let field_has_toto_attr = field.attrs.iter().any(|attr| match attr.meta {
                syn::Meta::Path(ref path) => {
                    !path.segments.is_empty() && path.segments.first().unwrap().ident == "toto"
                }
                _ => false,
            });
            if field_has_toto_attr {
                toto_fields.push_str(&format!(
                    " | {:?} is toto",
                    field.ident.unwrap().to_string()
                ));
            }
        }
    }
    quote! {
        impl Log for #type_name {
            fn log_derive(&self, input: &str) {
                println!("[derive proc macro] In derive proc macro: {}{}", input, #toto_fields);
            }
        }
    }
    .into()
}

/// Note attribute proc macro replace the whole item
/// Contrary to derive proc macro which adds new code to the derived item
/// This macro will just add a println at the beginning of the function
#[proc_macro_attribute]
pub fn attribute_log(args: TokenStream, item: TokenStream) -> TokenStream {
    let attr_arg = args.to_string();
    let syn::ItemFn {
        // The function signature
        sig,
        // The visibility specifier of this function
        vis,
        // The function block or body
        block,
        // Other attributes applied to this function
        attrs,
    } = syn::parse(item).unwrap();

    let stmts = block.stmts;
    let func_ident = sig.ident.to_string();

    quote!(
        // We reconstruct the function with our added coded

        // We reapply all the attributes on the function and add our custom statement
        #(#attrs)*
        // Reconstruct function declaration
        #vis #sig {
            println!("[attribute proc macro] Called the function {:?} with attribute macro arg \"{}\"", #func_ident, #attr_arg);
            #(#stmts)*
        }
    )
    .into()
}
