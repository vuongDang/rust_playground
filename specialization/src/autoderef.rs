//! Autoderef for specialization
//! https://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html
//!
//! We implement autoderef with a tag system so that a value can be passed instead of just a ref

#![allow(dead_code)]
use std::fmt::{Debug, Display};

// A custom type which are used as input
// We use a custom type to avoid interference
// with other blanket implementation such as Display for String
struct InputType<T>(T);

// The resulting type we're trying to convert to using specialization
// from different `InputType`
struct OutputType(String);

struct TagString;
struct TagDisplay;
struct TagDebug;

// Most specific impl
trait WithString {
    fn tag(&self) -> TagString;
}

impl WithString for &&InputType<String> {
    fn tag(&self) -> TagString {
        TagString
    }
}

impl TagString {
    fn create(self, input: InputType<String>) -> OutputType {
        println!("with string");
        OutputType(input.0)
    }
}

// 2nd specific impl
trait WithDisplay {
    fn tag(&self) -> TagDisplay;
}

impl<T: Display> WithDisplay for &InputType<T> {
    fn tag(&self) -> TagDisplay {
        TagDisplay
    }
}

impl TagDisplay {
    fn create<T: Display>(self, input: InputType<T>) -> OutputType {
        println!("with display");
        OutputType(format!("{}", input.0))
    }
}

// Most generic impl
trait WithDebug {
    fn tag(&self) -> TagDebug;
}

impl<T: Debug> WithDebug for InputType<T> {
    fn tag(&self) -> TagDebug {
        TagDebug
    }
}

impl TagDebug {
    fn create<T: Debug>(self, input: InputType<T>) -> OutputType {
        println!("with debug");
        OutputType(format!("{:?}", input.0))
    }
}

macro_rules! foo {
    ($e:expr) => {
        (&&&$e).tag().create($e)
    };
}

pub fn main() {
    let w1 = InputType(String::from("toto"));
    let w2 = InputType(3);
    let w3 = InputType(["a", "b"]);

    foo!(w1);
    foo!(w2);
    foo!(w3);
}
