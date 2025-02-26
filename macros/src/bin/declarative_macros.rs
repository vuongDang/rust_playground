// We try to do the macro from: https://veykril.github.io/tlborm/decl-macros/macros-practical.html
// This is a macro to compute recurrence that will be invoked with a form :
// a[n] = (sequence of initial values separated by a comma) ; recurrence expression using a[n-i]
// Example invocation for Fibonacci sequence: a[n] = 0, 1 ; a[n-2] + a[n-1]

// Trick: don't hesistate to use another macro to do meta stuff
// such as counting the number of members in a sequence of expressions
macro_rules! count_exprs {
    () => {
        0
    };
    ($expr1:expr) => {
        1
    };
    ($head:expr, $($tail:expr),*) => {
        1 + count_exprs!($($tail),*)
    };
}

macro_rules! recurrence {
    // Trick: To prevent a macro from being hygienic you can pull identifiers
    // from the macro invocation by identifying ident (here the array name and the index name)
    // This is useful since we can use these identifiers in the $recur expression
    ( $array:ident[$index:ident]: $sty:ty = $($inits:expr),+ ; $recur:expr ) => {{
        const NB_OPERANDS: usize = count_exprs!($($inits),+);

        struct Fib {
            last: [$sty; NB_OPERANDS],
            pos: usize,
        }

        impl Iterator for Fib {
            type Item = $sty;

            fn next(&mut self) -> Option<Self::Item> {
                let pos = self.pos;
                self.pos += 1;

                if pos < NB_OPERANDS {
                    Some(self.last[pos])
                } else {
                    let next = {
                        let $index = NB_OPERANDS;
                        let $array = &self.last;
                        $recur
                    };
                    for i in 0..(NB_OPERANDS - 1) {
                        self.last.swap(i, i+1);
                    }
                    self.last[NB_OPERANDS - 1] = next;
                    Some(next)
                }
            }
        }

        let last = [$($inits),+];

        Fib {
            last,
            pos: 0,
        }
    }};
}

fn main() {
    // An example of which kind of macro we want to create
    let mut fib_no_macro = {
        /// A struct to hold a lazy iterator of the Fibonacci sequence
        struct Fib {
            /// Last two values of the fib sequence
            last: [u64; 2],
            /// Current position
            pos: usize,
        }

        impl Iterator for Fib {
            type Item = u64;

            fn next(&mut self) -> Option<Self::Item> {
                let pos = self.pos;
                self.pos += 1;
                match pos {
                    0 => Some(self.last[0]),
                    1 => Some(self.last[1]),
                    _ => {
                        let temp = self.last;
                        let next = temp[0] + temp[1];
                        self.last = [temp[1], next];
                        Some(next)
                    }
                }
            }
        }

        Fib {
            last: [0, 1],
            pos: 0,
        }
    };

    // `fib_macro` should contain code similar to `fib_no_macro`
    let mut fib_macro = recurrence![a[n]: u64 = 0, 1 ; a[n-2] + a[n-1]];
    for _ in 0..10 {
        assert_eq!(fib_no_macro.next(), fib_macro.next());
    }

    println!(
        "no_macro: {:?}",
        fib_no_macro.take(10).collect::<Vec<u64>>()
    );
    println!("macro: {:?}", fib_macro.take(10).collect::<Vec<u64>>());
}
