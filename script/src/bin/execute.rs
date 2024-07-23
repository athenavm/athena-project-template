//! An end-to-end example of using the Athena SDK to run a program.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --package fibonacci-script --bin execute --release
//! ```
use athena_interface::MockHost;
use athena_sdk::{AthenaStdin, ExecutionClient};
use clap::Parser;

/// The ELF (executable and linkable format) file for the Athena RISC-V VM.
///
/// This file is generated by running `cargo athena build` inside the `program` directory.
pub const FIBONACCI_ELF: &[u8] = include_bytes!("../../../program/elf/fibonacci-program");

/// The arguments for the run command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct RunArgs {
    #[clap(long, default_value = "20")]
    n: u32,

    #[clap(long, default_value = "false")]
    evm: bool,
}

fn main() {
    // Setup the logger.
    athena_sdk::utils::setup_logger();

    // Parse the command line arguments.
    let args = RunArgs::parse();

    // Setup the execution client.
    let client = ExecutionClient::new();

    // Setup the program.
    // let (pk, vk) = client.setup(FIBONACCI_ELF);

    // Setup the inputs.
    let mut stdin = AthenaStdin::new();
    stdin.write(&args.n);

    println!("n: {}", args.n);

    // Run the program.
    let (mut output, _) = client
        .execute::<MockHost>(FIBONACCI_ELF, stdin, None, None, None)
        .expect("failed to run program");
    println!("Successfully executed program!");
    println!("fib(n): {}", output.read::<u32>());
}
