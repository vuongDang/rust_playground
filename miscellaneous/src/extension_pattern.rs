//! The orphan rule prevents one to impl external trait on external type.
//! This rule is used to enforce coherence for the compiler. Compiler coherence
//! means that there is at most one implementation of a trait for any given type.
//! Therefore what you can do is:
//! - internal trait + internal type
//! - internal trait + external type
//! - external trait + internal type
//!
//! Patterns to solve external trait + external struct are:
//! - "new type pattern" to make an external type into an internal one using a wrapper
//! - "extension pattern" by extending an external trait with an internal trait
//!
//! https://www.youtube.com/watch?v=qrf52BVaZM8

use std::{collections::HashSet, fmt::Display, hash::Hash};

// An extension to the Display trait
trait DisplayExt: Display {
    fn pretty_display(&self) -> String {
        format!("pretty: {}", self)
    }
}

impl<T: Display> DisplayExt for T {}

// More complex version taken from iterator
struct UniqueIterator<I>
where
    I: Iterator,
    I::Item: Eq + Hash + Clone,
{
    iter: I,
    has_seen: HashSet<I::Item>,
}

trait IteratorExt: Iterator {
    fn unique(self) -> UniqueIterator<Self>
    where
        Self: Sized,
        Self::Item: Eq + Hash + Clone,
    {
        UniqueIterator {
            iter: self,
            has_seen: HashSet::new(),
        }
    }
}

impl<I> Iterator for UniqueIterator<I>
where
    I: Iterator,
    I::Item: Eq + Hash + Clone,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.find(|item| self.has_seen.insert(item.clone()))
    }
}

impl<I: Iterator> IteratorExt for I {}

pub fn main() {
    let s = String::from("toto");
    println!("{}", s.pretty_display());

    let v: Vec<_> = vec![1, 1, 2, 2, 3].into_iter().unique().collect();
    println!("unique vec: {:?}", v);
}
