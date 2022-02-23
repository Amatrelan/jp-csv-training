mod cli;
mod csv_struct;
mod training;

use std::io::*;
use std::{env, process};

use clap::{CommandFactory, Parser};
use cli::{print_completions, Cli, Commands};
use csv_struct::CSVStruct;
use log::info;
use simplelog::{ColorChoice, Config, LevelFilter, TermLogger, TerminalMode};
use training::Training;

fn main() {
    let mut logging_level = LevelFilter::Error;

    if let Ok(log_level) = env::var("LOGLEVEL") {
        match log_level.to_lowercase().as_str() {
            "trace" => logging_level = LevelFilter::Trace,
            "debug" => logging_level = LevelFilter::Debug,
            "info" => logging_level = LevelFilter::Info,
            "warn" => logging_level = LevelFilter::Warn,
            "error" => logging_level = LevelFilter::Error,
            _ => eprintln!("Defaulting ERROR loglevel"),
        }
    }

    if let Err(e) =
        TermLogger::init(logging_level, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)
    {
        eprintln!("Error initializing Terminal Logger: {:?}", e);
        process::exit(1)
    }

    let args: Cli = Cli::parse();

    match args.command {
        Commands::Add(add) => {
            println!("Adding {:?}\n", add);
            unimplemented!("Store is not yet implemented.")
        }
        Commands::Train { csv_file } => {
            let mut training_material: Training;
            if let Some(csvfile) = csv_file {
                info!("Passed CSV file `{}` as argument, using it as training material.", csvfile);
                training_material = Training::from_csv(csvfile);

                while training_material.len() > 0 {
                    let (data, language) = training_material.get_random().unwrap();
                    match language {
                        training::Language::Japanese => {
                            let mut input = String::new();
                            println!("> {}", data.csv_data.japanese);
                            stdin().read_line(&mut input).expect("Failed to read input");

                            if input.trim().to_lowercase() == data.csv_data.english.to_lowercase() {
                                println!("Correct!");
                                training_material.mark_done(&data, language);
                            }
                        }
                        training::Language::English => {
                            let mut input = String::new();
                            println!("> {}", data.csv_data.english);
                            stdin().read_line(&mut input).expect("Failed to read input");

                            if input.trim() == data.csv_data.japanese {
                                println!("Correct!");
                                training_material.mark_done(&data, language);
                            }
                        }
                    }
                }

                println!("Finished training.")
            }
        }
        Commands::Import { filepath: _ } => unimplemented!("Store is not yet implemented."),
        Commands::Completions { generator } => {
            let mut cmd = Cli::command();
            eprintln!("Generating completions for {:?}", generator);
            print_completions(generator.unwrap(), &mut cmd);
        }
    }
}
