use clap::Parser;
use std::env;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `echo`
pub struct Args {
    /// config path
    #[arg(required(true))]
    pub conf: String,

    /// debug mode
    #[arg(short('d'))]
    pub debug_mode: bool,
}

#[allow(dead_code)]
pub fn print_parsed_args() {
    // command line arguments
    // by clap
    let args = Args::parse();
    println!(
        "{} {}",
        args.conf,
        if args.debug_mode { "debug mode" } else { "" }
    );

    // by std::env
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}

#[allow(dead_code)]
pub fn get_args() -> Args {
    Args::parse()
}
