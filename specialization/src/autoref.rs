//! Using autoref to do specialization
//! https://github.com/dtolnay/case-studies/blob/master/autoref-specialization/README.md

#![allow(dead_code)]
use std::fmt::{Debug, Display};

#[derive(Debug)]
struct DebugType {
    msg: String,
}

struct MyType {
    msg: String,
}

impl MyType {
    // This function is the default
    fn from_display<T: Display>(msg: T) -> Self {
        println!("Built with [from_display]");
        MyType {
            msg: msg.to_string(),
        }
    }

    // This function should be prioritized when possible
    fn from_debug<T: Debug>(msg: T) -> Self {
        println!("Built with [from_debug]");
        MyType {
            msg: format!("{:?}", msg),
        }
    }
}

trait DebugToMyType {
    fn convert(&self) -> MyType;
}

// We implement for &T to give priority to Display type
impl<T: Debug> DebugToMyType for &T {
    fn convert(&self) -> MyType {
        MyType::from_debug(self)
    }
}

trait DisplayToMyType {
    fn convert(&self) -> MyType;
}

impl<T: Display> DisplayToMyType for T {
    fn convert(&self) -> MyType {
        MyType::from_display(self)
    }
}

macro_rules! convert_to_mytype {
    ($msg:expr) => {
        (&$msg).convert();
    };
}

pub fn main() {
    let debug_var = DebugType {
        msg: String::from("toto "),
    };
    convert_to_mytype!("withdisplay type");
    convert_to_mytype!(debug_var);
    println!("{:?}", debug_var);
}
