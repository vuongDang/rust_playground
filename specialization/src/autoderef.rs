//! Autoderef for specialization
//! https://lukaskalbertodt.github.io/2019/12/05/generalized-autoref-based-specialization.html

use std::fmt::{Debug, Display};

struct Wrap<T>(T);

trait WithString {
    fn foo(&self); 
}

impl WithString for &&Wrap<String> { 
    fn foo(&self) { println!("with String") }
}

trait WithDebug {
    fn foo(&self) ;
}

impl<T: Debug> WithDebug for Wrap<T> { 
    fn foo(&self) { println!("with Debug") }
}

trait WithDisplay {
    fn foo(&self); 
}

impl<T: Display> WithDisplay for &Wrap<T>{
    fn foo(&self) { println!("with Display") }
}


macro_rules! foo {
    ($e:expr) => {
        (&&&$e).foo()
    }
}

pub fn main() {
    let w1 = Wrap(String::from("toto"));
    let w2 = Wrap(3);
    let w3 = Wrap(["a", "b"]);

    foo!(w1);
    foo!(w2);
    foo!(w3);
}
