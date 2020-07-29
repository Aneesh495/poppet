//! Poppet Command Line Interface.
//!
//! Copyright (c) 2026 Aneesh Krishna Parthasarathy
//! Licensed under the ISC License.

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "High-Integrity Pressure Protection System (HIPPS) in Befunge and Rust")]
struct Args {
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let _args = Args::parse();
    println!("Poppet HIPPS Safety Executive initialized.");
}
