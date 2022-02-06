use std::io;
use rand::Rng;

pub fn ram(
    stack: &mut Vec<f64>,
    cmd: Vec<&str>,
    statement: &str,
    vars: &mut super::super::Vars,
    b: usize,
    l: u32,
) {
    if cmd[1] == "lx" || cmd[1] == "rv" || cmd[1] == "string" {
        if cmd.len() == 2 {
            if cmd[1] == "lx" {
                stack.push(vars.lx)
            }
            if cmd[1] == "rv" {
                stack.push(vars.rv)
            }
        } else {
            if cmd[1] == "lx" {
                if cmd[2] == "prev" {
                    vars.lx = stack[stack.len() - 1];
                } else {
                    vars.lx = super::errors::parse_float(cmd[2], b, l)
                }
            }
            if cmd[1] == "rv" {
                if cmd[2] == "prev" {
                    vars.rv = stack[stack.len() - 1];
                } else {
                    vars.rv = super::errors::parse_float(cmd[2], b, l)
                }
            }
            if cmd[1] == "string" {
                let lits: Vec<&str> = statement.split(">>").collect();
                vars.string = lits[1].to_string();
            }
        }
    } else {
        stack.push(super::errors::parse_float(cmd[1], b, l))
    }
}

pub fn pop(stack: &mut Vec<f64>, b: usize, l: u32) {
    if stack.len() < 1 {
        super::errors::stack_len_error(b, l);
    }
    stack.pop();
}

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

pub fn random(stack: &mut Vec<f64>, statement: &str, b: usize, l: u32) {
    let rand_cmd: Vec<&str> = statement.split(">>").collect();
    let numbers: Vec<&str> = rand_cmd[1].split(",").collect();

    let mut rng = rand::thread_rng();
    let random_num = rng.gen_range(
        super::errors::parse_float(numbers[0].trim(), b, l)..super::errors::parse_float(numbers[1].trim(), b, l),
    );
    stack.push(random_num);
}
