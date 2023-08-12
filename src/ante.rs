use std::fs;
use regex::Regex;

pub fn search_for_files(path: &str) -> Vec<String> {
    let mut paths = Vec::new();

    let Ok(entries) = fs::read_dir(&path) else {
        return paths;
    };

    for entry in entries {
        let Ok(entry) = entry else {
            continue;
        };

        let Ok(metadata) = entry.metadata() else {
            continue;
        };
        
        let path = entry.path().to_str().unwrap().to_string();

        if metadata.is_dir() {
            paths.append(&mut search_for_files(entry.path().to_str().unwrap()));
        }else {
            paths.push(path);
        }
    }

    return paths;
}

pub fn file_contains_string(path: &String, text: &String) -> bool {
    let Ok(data) = fs::read_to_string(path) else {
        return false;
    };
    
    return data.contains(text);
}

pub fn file_contains_regex(path: &String, regex: &Regex) -> bool {
    let Ok(data) = fs::read_to_string(path) else {
        return false;
    };
    
    return regex.is_match(&data);
}
