//! Pattern to implement a state machine which soundness
//! can be checked at compile time with the type checker

#![allow(dead_code)]
use std::marker::PhantomData;

trait State {}

// Transition from state: A -> B -> C -> A ...
struct A(String);
struct B(String);
struct C(String);

impl State for A {}
impl State for B {}
impl State for C {}

struct Machine<S: State> {
    state: S,
    _marker: PhantomData<S>,
}

impl<S: State> Machine<S> {
    fn new() -> Machine<A> {
        Machine {
            state: A(String::from("a")),
            _marker: PhantomData,
        }
    }
}

impl Machine<A> {
    fn next(self) -> Machine<B> {
        let b = B(self.state.0 + "b");
        Machine {
            state: b,
            _marker: PhantomData,
        }
    }
}

impl Machine<B> {
    fn next(self) -> Machine<C> {
        let c = C(self.state.0 + "c");
        Machine {
            state: c,
            _marker: PhantomData,
        }
    }
}

impl Machine<C> {
    fn next(self) -> Machine<A> {
        let a = A(self.state.0 + "a");
        Machine {
            state: a,
            _marker: PhantomData,
        }
    }
}

pub fn main() {
    let m = Machine::<A>::new();
    let m = m.next();
    let m = m.next();
    let m = m.next();
    let m = m.next();
    println!("{}", m.state.0)
}
