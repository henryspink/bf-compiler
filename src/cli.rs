use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "Brainf Compiler")]
#[command(version = "0.1")]
#[command(about = "Does awesome things", long_about = None)]
#[command(propagate_version = true)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    // Compile a given brainf file into the given target
    Compile(CompileArgs),
    // Run a given brainf file
    Run(RunArgs),
    // Test a given brainf file
    Test(TestArgs),
}

#[derive(Args, Clone, Debug)]
pub struct CompileArgs {
    pub path: PathBuf,
    #[arg(short, long, default_value_t = String::from("binary"))]
    pub emit: String,
    #[arg(short, long, default_value_t = false)]
    pub no_optimize: bool,
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,
}

#[derive(Args, Clone)]
pub struct RunArgs {
    pub path: PathBuf,
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,
}

#[derive(Args, Clone)]
pub struct TestArgs {
    pub path: PathBuf,
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,
}
