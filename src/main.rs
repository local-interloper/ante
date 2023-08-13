mod args;
mod ante;

use args::Args;
use clap::Parser;
use regex::Regex;
use std::process::ExitCode;

use crate::ante::{get_file_regex_matches, get_file_text_matches, print_matches, search_for_files};

fn main() -> ExitCode {
    let args = Args::parse();

    if let Some(text) = args.text {
        for path in search_for_files(&args.path, args.max_file_size).iter() {
            let matches = get_file_text_matches(path, &text);

            if matches.len() == 0 {
                continue;
            }

            print_matches(path, &matches, args.paths_only);
        }

        return ExitCode::SUCCESS;
    }


    if let Some(regex) = args.regex {
        let regex = Regex::new(&regex);

        let Ok(regex) = regex else {
            eprintln!("Invalid regular expression");
            return ExitCode::FAILURE;
        };

        for path in search_for_files(&args.path, args.max_file_size).iter() {
            let matches = get_file_regex_matches(path, &regex);

            if matches.len() == 0 {
                continue;
            }

            print_matches(path, &matches, args.paths_only);
        }

        return ExitCode::SUCCESS;
    }

    ExitCode::FAILURE
}