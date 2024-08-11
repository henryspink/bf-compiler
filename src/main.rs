use clap::Parser;
mod cli;

use crate::cli::{CliArgs, Commands};

mod compiler;
mod interpreter;
mod validator;

fn main() {
    let cli_args = CliArgs::parse();

    match cli_args.command {
        Commands::Compile(args) => {
            println!("Compiling file: {:?} {:?}", args.path, args);
            match compiler::compile(args) {
                Ok(_) => println!("Compilation successful"),
                Err(e) => println!("Compilation failed: {}", e),
            }
        }
        Commands::Run(args) => {
            println!("Running file: {:?}", args.path);
            match interpreter::interpret(args) {
                Ok(_) => println!("Interpretion successful"),
                Err(e) => println!("Interpretation failed: {}", e),
            }
        }
        Commands::Test(args) => {
            println!("Testing file: {:?}", args.path);
            // validator::validate(args);
        }
    }
}
