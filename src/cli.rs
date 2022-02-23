use std::io;

use clap::{Command, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};

use crate::CSVStruct;

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Add new element to training list
    Add(CSVStruct),
    /// Train with existing dataset
    Train { csv_file: Option<String> },
    /// Import csv material to dataset
    Import { filepath: String },
    /// Generate shell completions
    Completions {
        #[clap(long = "generate", arg_enum)]
        generator: Option<Shell>,
    },
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, trailing_var_arg = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

pub fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout())
}
