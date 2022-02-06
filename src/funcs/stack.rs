use rand::Rng;
use std::io;

pub fn ram(
    stack: &mut Vec<f64>,
    cmd: Vec<&str>,
    statement: &str,
    vars: &mut super::super::Vars,
    b: usize,
    l: u32,
) {
    if cmd[1] == "lx" || cmd[1] == "rv" || cmd[1] == "string" || cmd[1] == "vec" {
        if cmd.len() < 2 {
            super::errors::args_error(b, l)
        }
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
            if cmd[1] == "vec" {
                if cmd.len() < 5 {
                    super::errors::args_error(b, l);
                } else {
                    if cmd[2] == "int" {
                        let lits: Vec<&str> = statement.split(">>").collect();
                        let str_vec: String = lits[1].trim().to_string();
                        let slice = &str_vec[1..str_vec.len() - 1];
                        let data_vec: Vec<&str> = slice.split(",").collect();
                        for item in data_vec {
                            vars.num_vec.push(super::errors::parse_float(item, b, l));
                        }
                    } else if cmd[2] == "str" {
                        let lits: Vec<&str> = statement.split(">>").collect();
                        let data_vec: String = lits[1].trim().to_string();
                        let slice = &data_vec[1..data_vec.len() - 1];
                        let str_vec: Vec<&str> = slice.split(",").collect();
                        for item in str_vec {
                            vars.str_vec.push(item.trim().to_string());
                        }
                    } else {
                        super::errors::args_error(b, l)
                    }
                }
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
        super::errors::parse_float(numbers[0].trim(), b, l)
            ..super::errors::parse_float(numbers[1].trim(), b, l),
    );
    stack.push(random_num);
}
