//! Playing around macros
//! - [The Little Book of Rust Macros](https://veykril.github.io/tlborm/syntax-extensions.html)
//! Macros can be expanded and shown via:
//! -`rustc +nightly -Zunpretty=expanded`
//! - cargo plugin: `cargo-expand` (a wrapper around rustc option mentioned above)
#![allow(dead_code)]

use macros::*;

pub trait Log {
    fn log_derive(&self, input: &str);
}

// This is the custome derive proc macro
#[derive(Log)]
struct DeriveProcMacroStruct {
    field1: u8,
    #[toto]
    field2: u8,
}

fn main() {
    toto(4);
    let dpms = DeriveProcMacroStruct {
        field1: 3,
        field2: 4,
    };
    dpms.log_derive("haha");
}

// Attribute macro that will log when the function is called
#[attribute_log(attr_arg)]
fn toto(val: u8) -> u8 {
    // This is the function proc macro
    function_macro_log!("proc macro: TIME in function \"toto\"");
    val
}
