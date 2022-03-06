use rand::Rng;
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
        if cmd[1].trim() == "string" {
            vars.string = input;
        } else {
            let number: f64 = super::errors::parse_float(input.trim(), b, l);
            if cmd[1].trim() == "lx" {
                vars.lx = number
            }
            if cmd[1].trim() == "rv" {
                vars.rv = number
            }
        }
    }
}

pub fn stdfs(vars: &mut super::super::Vars, cmd: Vec<&str>, _statement: &str, b: usize, l: u32) {
    if cmd.len() < 3 {
        super::errors::args_error(b, l);
    } else if cmd.len() == 3 {
        if cmd[2].trim() == "string" || cmd[2].trim() == "lxstring" {
            if cmd[2].trim() == "string" {
                let mut file = fs::File::open(vars.string.trim()).expect(
                    format!(
                        "No such file exists block::{}:line:{}",
                        b.to_string().trim(),
                        l.to_string()
                    )
                    .trim(),
                );
                let mut contents = String::new();
                file.read_to_string(&mut contents)
                    .expect("something went wrong");
                vars.string = contents;
            } else if cmd[2].trim() == "lxstring" {
                let mut file = fs::File::open(vars.lxstring.trim()).expect(
                    format!(
                        "No such file exists block::{}:line:{}",
                        b.to_string().trim(),
                        l.to_string()
                    )
                    .trim(),
                );
                let mut contents = String::new();
                file.read_to_string(&mut contents)
                    .expect("something went wrong");
                vars.string = contents;
            }
        } else {
            super::errors::args_error(b, l);
        }
    } else {
        if cmd[1] == "open" {
            let lits: Vec<&str> = _statement.split(">>").collect();
            let mut file = fs::File::open(lits[1].trim()).expect(
                format!(
                    "No such file exists block::{}:line:{}",
                    b.to_string().trim(),
                    l.to_string()
                )
                .trim(),
            );
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("something went wrong");
            vars.string = contents;
        } else {
            super::errors::args_error(b, l);
        }
    }
}

pub fn random(
    vars: &mut super::super::Vars,
    cmd: Vec<&str>,
    stack: &mut Vec<f64>,
    statement: &str,
    b: usize,
    l: u32,
) {
    if cmd.len() < 3 {
        super::errors::args_error(b, l);
    }
    let rand_cmd: Vec<&str> = statement.split(">>").collect();
    let args: Vec<&str> = rand_cmd[1].split(",").collect();

    let mut rng = rand::thread_rng();
    let random_num =
        rng.gen_range(ret_float(args[0].trim(), vars, b, l)..ret_float(args[1].trim(), vars, b, l));
    stack.push(random_num);
}

fn ret_float(str_num: &str, vars: &mut super::super::Vars, b: usize, l: u32) -> f64 {
    if str_num == "lx" || str_num == "rv" {
        if str_num == "lx" {
            vars.lx
        } else {
            vars.rv
        }
    } else {
        super::errors::parse_float(str_num.trim(), b, l)
    }
}

pub fn parse_int(vars: &mut super::super::Vars, cmd: Vec<&str>, b: usize, l: u32) {
    // parse int lx string
    // parse int lx lxstring

    // parse int rv string
    // parse int rv lxstring

    if cmd[2] == "lx" {
        if cmd[3].trim() == "string" {
            vars.lx = super::errors::parse_float(vars.string.trim(), b, l);
        } else if cmd[3].trim() == "lxstring" {
            vars.lx = super::errors::parse_float(vars.lxstring.trim(), b, l);
        } else {
            super::errors::args_error(b, l);
        }
    } else if cmd[2] == "rv" {
        if cmd[3].trim() == "string" {
            vars.rv = super::errors::parse_float(vars.string.trim(), b, l);
        } else if cmd[3].trim() == "lxstring" {
            vars.rv = super::errors::parse_float(vars.lxstring.trim(), b, l);
        } else {
            super::errors::args_error(b, l);
        }
    } else {
        super::errors::args_error(b, l);
    }
}
