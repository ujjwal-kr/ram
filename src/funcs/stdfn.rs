use std::fs;
use std::io;
use std::io::prelude::*;

pub fn stdin(vars: &mut super::super::Vars, cmd: Vec<&str>, b: usize, l: u32) {
    if cmd.len() != 2 {
        super::errors::args_error(b, l);
    } else {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("something went wrong");
        if cmd[1] == "string" {
            vars.string = input;
        } else {
            let number: f64 = super::errors::parse_float(input.trim(), b, l);
            if cmd[1] == "lx" {
                vars.lx = number
            }
            if cmd[1] == "rv" {
                vars.rv = number
            }
        }
    }
}

pub fn stdfs(vars: &mut super::super::Vars, cmd: Vec<&str>, statement: &str, b: usize, l: u32) {
    if cmd.len() < 4 {
        super::errors::args_error(b, l);
    } else {
        if cmd[1] == "open" {
            let lits: Vec<&str> = statement.split(">>").collect();
            let mut file = fs::File::open(lits[1].trim()).expect("No such file exists");
            let mut contents = String::new();
            file.read_to_string(&mut contents).expect("something went wrong");
            vars.string = contents;
        }
    }
}
