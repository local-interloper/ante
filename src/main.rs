mod args;
mod ante;

use args::Args;
use clap::Parser;
use regex::Regex;
use std::process::ExitCode;

use crate::ante::{file_contains_regex, file_contains_string, search_for_files};

fn main() -> ExitCode {
    let args = Args::parse();

    if let Some(text) = args.text {
        search_for_files(&args.path)
            .iter()
            .filter(|path| file_contains_string(path, &text))
            .for_each(|path| {
                println!("{}", path);
            });
       
        return ExitCode::SUCCESS;
    }

    if let Some(regex) = args.regex {
        let regex = Regex::new(&regex);

        let Ok(regex) = regex else {
            eprintln!("Invalid regular expression");
            return ExitCode::FAILURE;
        };

        search_for_files(&args.path)
            .iter()
            .filter(|path| file_contains_regex(path, &regex))
            .for_each(|path| {
                println!("{}", path);
            });

        return ExitCode::SUCCESS;
    }
    
    ExitCode::FAILURE
}