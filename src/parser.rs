use regex::Regex;
use std::collections::HashMap;
use std::io::prelude::*;
use std::{fs, path::Path, process};

#[derive(Debug)]
pub struct LabelMap {
    pub map: HashMap<String, usize>,
    pub instructions: Vec<String>,
}

pub fn parse_lines(p_lines: Vec<&str>) -> LabelMap {
    let mut _final_lines: Vec<&str> = vec![];
    _final_lines = p_lines.clone();

    let mut final_contents = String::new();

    let imports = has_includes(p_lines);

    // check if the files exist
    for path in imports {
        if !Path::new(path).exists() {
            println!("File '{}' not found", path);
            process::exit(1);
        }

        // read the file

        let mut file = fs::File::open(path).unwrap_or_else(|_| { panic!("{}", format!("Error opening '{}'", path).trim().to_string()) });
        let mut contents = String::new();

        file.read_to_string(&mut contents)
            .unwrap_or_else(|_| { panic!("{}", format!("Error reading '{}'", path).trim().to_string()) });

        final_contents += &contents;
        final_contents += "\n";
    }

    let mut final_import_vec: Vec<&str> = final_contents.split('\n').collect();
    _final_lines.append(&mut final_import_vec);
    let program: LabelMap = populate_labels(_final_lines);
    program
}

// takes in all the lines and send imports of a file
pub fn has_includes(p_lines: Vec<&str>) -> Vec<&str> {
    let mut has_included: Vec<&str> = vec![];
    for mut line in p_lines {
        line = line.trim();
        if !line.is_empty() {
            let cmd = line.split_whitespace().collect::<Vec<&str>>();
            if cmd[0] == "include" {
                let includes = &cmd[1].split('\"').collect::<Vec<&str>>()[1];
                has_included.push(includes);
            }
        }
    }

    has_included
}

pub fn populate_labels(p_lines: Vec<&str>) -> LabelMap {
    let mut map: HashMap<String, usize> = HashMap::new();
    let mut instructions: Vec<String> = vec![];
    let exp = Regex::new(r"^[a-zA-Z0-9_]+:$").unwrap();
    let mut i = 0u32;

    for mut line in p_lines {
        let mut incr: bool = true;
        line = line.trim();
        if !line.is_empty() && line.split_whitespace().collect::<Vec<&str>>()[0] == "include" {
            incr = false;
            line = "";
        } else if line.len() > 1 && &line[..2] == "//" {
            incr = false;
            line = "";
        }
        if exp.is_match(line) {
            if i == 0 && line != "main:" {
                panic!("No main label at the beginning of the file.");
            }
            map.insert(line.to_string(), i as usize);
        } else if !line.is_empty() && incr {
            i += 1;
            instructions.push(line.to_string())
        }
    }

    LabelMap { map, instructions }
}
