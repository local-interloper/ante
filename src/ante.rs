use std::fs;
use std::fs::metadata;
use std::path::Path;
use regex::Regex;

pub struct Match {
    pub line_no: usize,
    pub text: String,
}

pub fn search_for_files(path: &str, max_file_size: Option<u64>) -> Vec<String> {
    let mut paths = Vec::new();

    if Path::new(&path).is_file() {
        paths.push(path.to_string());
        return paths;
    }

    let Ok(entries) = fs::read_dir(&path) else {
        return paths;
    };

    for entry in entries {
        let Ok(entry) = entry else {
            continue;
        };

        let path = entry.path().to_str().unwrap().to_string();

        let Ok(metadata) = metadata(&path) else {
            continue;
        };


        if metadata.is_dir() {
            paths.append(&mut search_for_files(entry.path().to_str().unwrap(), max_file_size));
        } else {
            paths.push(path);
        }
    }

    return paths;
}

pub fn get_file_text_matches(path: &String, text: &String) -> Vec<Match> {
    let mut result = Vec::new();

    let Ok(data) = fs::read_to_string(path) else {
        return result;
    };

    for (line_index, line) in data.lines().enumerate() {
        if line.contains(text) {
            result.push(Match {
                line_no: line_index + 1,
                text: line.to_string(),
            })
        }
    }

    return result;
}

pub fn get_file_regex_matches(path: &String, regex: &Regex) -> Vec<Match> {
    let mut result = Vec::new();

    let Ok(data) = fs::read_to_string(path) else {
        return result;
    };

    for (line_index, line) in data.lines().enumerate() {
        if regex.is_match(line) {
            result.push(Match {
                line_no: line_index + 1,
                text: line.to_string(),
            })
        }
    }

    return result;
}

pub fn print_matches(path: &str, matches: &Vec<Match>, paths_only: bool) {
    println!("{}", path);

    if paths_only {
        return;
    }
   
    for m in matches {
        println!("{} | {}", m.line_no, m.text);
    }
}