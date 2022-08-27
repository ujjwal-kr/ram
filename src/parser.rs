use regex::Regex;
use std::collections::HashMap;
use std::io::prelude::*;
use std::{fs, path::Path, process};

pub fn parse_lines(p_lines: Vec<&str>) -> HashMap<String, Vec<String>> {
    let mut final_lines: Vec<&str> = vec![];
    for line in p_lines.clone() {
        final_lines.push(line);
    }

    let mut final_contents = String::new();

    let imports = has_includes(p_lines);

    // check if the files exist
    for path in imports {
        if !Path::new(path).exists() {
            println!("File '{}' not found", path);
            process::exit(1);
        }

        // read the file

        let mut file = fs::File::open(path).expect(format!("Error opening '{}'", path).trim());
        let mut contents = String::new();

        file.read_to_string(&mut contents)
            .expect(format!("Error reading '{}'", path).trim());

        final_contents += &contents;
        final_contents += "\n";
    }

    let final_import_vec: Vec<&str> = final_contents.split("\n").collect();
    for line in final_import_vec {
        final_lines.push(line)
    }
    let program = populate_labels(final_lines);
    program
}

// takes in all the lines and send imports of a file
pub fn has_includes(p_lines: Vec<&str>) -> Vec<&str> {
    let mut has_included: Vec<&str> = vec![];
    for mut line in p_lines {
        line = line.trim();
        if line != "" {
            let cmd = line.split_whitespace().collect::<Vec<&str>>();
            if cmd[0] == "include" {
                let includes = &cmd[1].split("\"").collect::<Vec<&str>>()[1];
                has_included.push(includes);
            }
        }
    }

    has_included
}

pub fn populate_labels(p_lines: Vec<&str>) -> HashMap<String, Vec<String>> {
    has_includes(p_lines.clone());
    let mut program: HashMap<String, Vec<String>> = HashMap::new();
    let mut current_key: String = String::new();
    let exp = Regex::new(r"^[a-zA-Z0-9_]+:$").unwrap();
    let mut i = 0u32;
    for mut line in p_lines {
        line = line.trim();
        if line != "" && line.split_whitespace().collect::<Vec<&str>>()[0] == "include" {
            line = ""
        }
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
