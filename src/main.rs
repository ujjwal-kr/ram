use regex::Regex;
use std::collections::HashMap;
use std::io::prelude::*;
use std::{env, f64, fs, io, path::Path, process};

mod funcs;
mod tests;
use funcs::{errors, jump, operations, print, stack, stdfn, var};

#[derive(Clone)]
pub struct Vars {
    pub lx: f64,
    pub rv: f64,
    pub string: String,
    pub lxstring: String,
    pub str_vec: Vec<String>,
    pub num_vec: Vec<f64>,
}

#[derive(Clone)]
pub struct HashVars {
    pub hash_str: HashMap<String, String>,
    pub hash_int: HashMap<String, f64>,
    pub hash_str_vec: HashMap<String, Vec<String>>,
    pub hash_int_vec: HashMap<String, Vec<f64>>,
}

fn populate_labels(p_lines: Vec<&str>) -> HashMap<String, Vec<String>> {
    let mut program: HashMap<String, Vec<String>> = HashMap::new();
    let mut current_key: String = String::new();
    let exp = Regex::new(r"^[a-zA-Z0-9]+:$").unwrap();
    let mut i = 0u32;
    for mut line in p_lines {
        line = line.trim();
        if exp.is_match(line) {
            if i == 0 && line != "main:" {
                panic!("No main label");
            }
            current_key = line.to_string();
            program.insert(line.to_string(), vec![]);
            line = "";
        }
        let mut new_vec: Vec<String> = vec![];
        match program.get(current_key.trim()) {
            Some(value) => new_vec = value.to_vec(),
            _ => panic!("program parsing error"),
        }
        if line != "" {
            new_vec.push(line.to_string());
        }
        if let Some(x) = program.get_mut(current_key.trim()) {
            *x = new_vec;
        }
        i = i + 1;
    }
    program
}

fn main() -> std::io::Result<()> {
    let mut filename = String::new();
    if env::args().nth(1).is_none() == true {
        println!("Welcome to the Ram stack-based programming language.");
        println!("Please enter a filename: ");
        io::stdin().read_line(&mut filename)?;
        filename = filename.trim().to_string();
    } else {
        if env::args().nth(1).unwrap() == "test" {
            tests::test();
            if Path::new("log.txt").exists() {
                fs::remove_file("log.txt").expect("");
                panic!("Tests failed");
            } else {
                process::exit(0);
            }
        } else {
            filename = env::args().nth(1).unwrap();
        }
    }

    let mut file = fs::File::open(filename.trim())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let p_lines: Vec<&str> = contents.split("\n").collect();
    let program: HashMap<String, Vec<String>> = populate_labels(p_lines);

    let vars = Vars {
        lx: 0.0,
        rv: 0.0,
        string: "".to_string(),
        lxstring: "".to_string(),
        num_vec: vec![],
        str_vec: vec![],
    };

    let mut hash_vars = HashVars {
        hash_str: HashMap::new(),
        hash_int: HashMap::new(),
        hash_int_vec: HashMap::new(),
        hash_str_vec: HashMap::new(),
    };
    match run_statement(program, "main:", vars, &mut hash_vars) {
        Ok(()) => (),
        _ => println!("Something went wrong"),
    }
    Ok(())
}

pub fn run_statement(
    program: HashMap<String, Vec<String>>,
    run_label: &str,
    vars: Vars,
    hash_vars: &mut HashVars,
) -> std::io::Result<()> {
    let mut line = 0u32;
    let mut block_number: usize = 1;
    
    let mut local_vars = Vars {
        lx: vars.lx,
        rv: vars.rv,
        string: vars.string,
        lxstring: vars.lxstring,
        num_vec: vars.num_vec,
        str_vec: vars.str_vec,
    };

    let run_block: Vec<String>;
    match program.get(run_label) {
        Some(value) => run_block = value.to_vec(),
        _ => panic!("Function not found")
    }

    let mut stack: Vec<f64> = vec![];
    for statement in run_block {
        let statement = statement.trim();
        line = line + 1;
        // skip tabs and split by spaces
        let mut cmd: Vec<&str> = statement.split(" ").collect();
        // comments
        let lits: Vec<&str> = cmd[0].trim().split("").collect();
        if lits[1] == "/" && lits[2] == "/" {
            cmd[0] = "//";
        }
        match cmd[0].trim() {
            "//" => (),
            "" => (),
            "print" => print::print(
                &mut stack,
                cmd,
                &mut local_vars,
                hash_vars,
                block_number,
                line,
            ),
            "printc" => print::printc(cmd, statement, block_number, line),
            "ram" => stack::ram(
                &mut stack,
                cmd,
                statement,
                &mut local_vars,
                hash_vars,
                block_number,
                line,
            ),
            "var" => var::var(
                &mut stack,
                cmd,
                statement,
                &mut local_vars,
                hash_vars,
                block_number,
                line,
            ),
            "move" => var::movefn(cmd, &mut local_vars, hash_vars, block_number, line),
            "str" => stack::strfn(&mut stack, &mut local_vars, cmd, block_number, line),
            "stdin" => stdfn::stdin(&mut local_vars, cmd, block_number, line),
            "stdfs" => stdfn::stdfs(&mut local_vars, cmd, statement, block_number, line),
            "pop" => stack::pop(&mut stack, block_number, line),
            "popall" => stack = vec![],
            "add" => operations::add(&mut stack, cmd, &mut local_vars, block_number, line),
            "sub" => operations::sub(&mut stack, block_number, line),
            "mul" => operations::mul(&mut stack, cmd, &mut local_vars, block_number, line),
            "div" => operations::div(&mut stack, block_number, line),
            "sqr" => operations::sqr(&mut stack, cmd, &mut local_vars, block_number, line),
            "sqrt" => operations::sqrt(&mut stack, cmd, &mut local_vars, block_number, line),
            "round" => operations::round(&mut stack, cmd, &mut local_vars, block_number, line),
            "avg" => operations::avg(&mut stack, block_number, line),
            "rand" => stdfn::random(
                &mut local_vars,
                cmd,
                &mut stack,
                statement,
                block_number,
                line,
            ),
            "split" => operations::split(cmd, statement, &mut local_vars, block_number, line),
            "parse" => stdfn::parse_int(&mut local_vars, cmd, block_number, line),
            "vec" => operations::vec_ops(
                &mut stack,
                cmd,
                statement,
                &mut local_vars,
                block_number,
                line,
            ),
            // "cmp" => operations::cmp(&mut stack, block_number, line),
            // "je" => jump::je(
            //     &mut stack,
            //     cmd,
            //     blocks,
            //     local_vars.clone(),
            //     hash_vars,
            //     block_number,
            //     line,
            // ),
            // "jne" => jump::jne(
            //     &mut stack,
            //     cmd,
            //     blocks,
            //     local_vars.clone(),
            //     hash_vars,
            //     block_number,
            //     line,
            // ),
            // "jgr" => jump::jgr(
            //     &mut stack,
            //     cmd,
            //     blocks,
            //     local_vars.clone(),
            //     hash_vars,
            //     block_number,
            //     line,
            // ),
            // "jsm" => jump::jsm(
            //     &mut stack,
            //     cmd,
            //     blocks,
            //     local_vars.clone(),
            //     hash_vars,
            //     block_number,
            //     line,
            // ),
            "jmp" => jump::jmp(
                cmd,
                program.clone(),
                local_vars.clone(),
                hash_vars,
                block_number,
                line,
            ),
            "halt" => process::exit(0),
            _ => {
                println!(
                    "Cant recognize command '{}' at b{}:l{}",
                    cmd[0],
                    block_number.to_string(),
                    line.to_string()
                );
                process::exit(1)
            }
        }
    }
    Ok(())
}
