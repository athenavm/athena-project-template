//! A simple program that takes a number `n` as input, and writes the `n-1`th and `n`th fibonacci
//! number as an output.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the VM.
#![no_main]
athena_vm::entrypoint!(main);

pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    let n = athena_vm::io::read::<u32>();

    if n > 186 {
        panic!(
            "This fibonacci program doesn't support n > 186, as it would overflow a 32-bit integer."
        );
    }

    // Compute the n'th fibonacci number, using normal Rust code.
    let mut a = 0u32;
    let mut b = 1u32;
    for _ in 0..n {
        let c = a + b;
        a = b;
        b = c;
    }

    athena_vm::io::write(&a);
    athena_vm::io::write(&b);
}
