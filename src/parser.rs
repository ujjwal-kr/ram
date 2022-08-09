use regex::Regex;
use std::collections::HashMap;
use std::io::prelude::*;
use std::{fs, io, path::Path, process};

#[derive(Debug)]
pub struct IncludeMapping {
    pub filename: String,
    pub has_included: Vec<String>,
}

#[derive(Debug)]
pub struct MappedProgram<'a> {
    pub p_lines: Vec<&'a str>,
    pub map: HashMap<String, Vec<String>>,
}

pub fn populate_labels(p_lines: Vec<&str>) -> HashMap<String, Vec<String>> {
    file_map_includes(p_lines.clone(), "main.rsm".to_string());
    let mut program: HashMap<String, Vec<String>> = HashMap::new();
    let mut current_key: String = String::new();
    let exp = Regex::new(r"^[a-zA-Z0-9_]+:$").unwrap();
    let mut i = 0u32;
    for mut line in p_lines {
        line = line.trim();
        if exp.is_match(line) {
            if i == 0 && line != "main:" {
                panic!("No main label at the beginning of the file.");
            }
            current_key = line.to_string();
            program.insert(line.to_string(), vec![]);
        } else if line != "" && &line[..2] != "//" {
            let mut _new_vec: Vec<String> = vec![];
            match program.get(current_key.trim()) {
                Some(value) => _new_vec = value.to_vec(),
                _ => {
                    println!("command '{}' not recognized", line);
                    process::exit(1)
                }
            }
            _new_vec.push(line.to_string());
            if let Some(x) = program.get_mut(current_key.trim()) {
                *x = _new_vec;
            }
            i += 1;
        }
    }
    program
}

// IMPORTS

// opens up files by discovering them through file_map_includes and traversing through them
pub fn traverse_includes(filename: String) -> MappedProgram<'static> {
    let mut final_lines: Vec<String> = vec![];
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    // open the file
    let mut file = fs::File::open(filename.trim())
        .expect(format!("Error opening {}", filename).trim());

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect(format!("Error reading {}", filename).trim());

    let p_lines: Vec<&str> = contents.split("\n").collect();
    let file_map: IncludeMapping = file_map_includes(p_lines, filename);
    

    MappedProgram {
        p_lines: vec![],
        map: map,
    }
}

// takes in all the lines and send IncludeMapping of a file
pub fn file_map_includes(p_lines: Vec<&str>, filename: String) -> IncludeMapping {
    let mut has_included: Vec<String> = vec![];
    for mut line in p_lines {
        line = line.trim();
        if line != "" {
            let cmd = line.split_whitespace().collect::<Vec<&str>>();
            if cmd[0] == "include" {
                let includes = &cmd[1].split("\"").collect::<Vec<&str>>()[1];
                has_included.push(includes.to_string());
            }
        }
    }

    IncludeMapping {
        filename: filename,
        has_included: has_included,
    }
}
