//! Higher-Rank Trait bounds are used to express generic lifetime for trait bounds
//! For example to express that a trait bound accept any lifetimes
//! Inspired from https://www.youtube.com/watch?v=6fwDwJodJrg
//! https://doc.rust-lang.org/nomicon/hrtb.html

#![allow(dead_code)]
use std::fmt::Display;


trait Formatter {
    fn format<T: Display>(&self, value: T) -> String;
}

struct SimpleFormatter;

impl Formatter for SimpleFormatter {
    fn format<T: Display>(&self, value: T) -> String {
        format!("{}", value)
    }
}

// If written like this the lifetime of the closure parameter has to be 
// at least as long as the lifetime of the closure itself.
// While this is safe this is not what we want.
// What we want to have is that the lifetime of the parameter just have to
// be at least as long as the execution of the closure.
fn apply_format<F>(formatter: F) -> impl Fn(&str) -> String where F: Formatter {
    move |s| formatter.format(s)
}

// Here the closure returned accept any kind of reference as input
fn apply_format_hrtb<F>(formatter: F) -> impl for <'a> Fn(&'a str) -> String where F: Formatter {
    move |s| formatter.format(s)
}

pub fn main() {

    let format = apply_format(SimpleFormatter);
    let format_hrtb = apply_format_hrtb(SimpleFormatter);
    
    // Weirdly enough this is compiling just fine without hrtb
    // There is something I have not understood 
    let s1 = String::from("haha");
    println!("{}", format(&s1));

    { 
        let s = String::from("toto");
        println!("{}", format(&s));
        println!("{}", format_hrtb(&s));
    }

    let s = String::from("tata");
    println!("{}", format(&s));
    println!("{}", format_hrtb(&s));
}

