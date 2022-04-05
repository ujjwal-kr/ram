use super::errors;

pub fn ram(
    stack: &mut Vec<f64>,
    cmd: Vec<&str>,
    statement: &str,
    vars: &mut super::super::Vars,
    hash_vars: &mut super::super::HashVars,
    b: usize,
    l: u32,
) {
    match cmd[1].trim() {
        "lx" => {
            if cmd.len() == 2 {
                stack.push(vars.lx)
            } else if cmd.len() >= 3 {
                match cmd[2].trim() {
                    "prev" => vars.lx = stack[stack.len() - 1],
                    _ => vars.lx = errors::parse_float(cmd[2], b, l),
                }
            } else {
                errors::args_error(b, l);
            }
        }
        "rv" => {
            if cmd.len() == 2 {
                stack.push(vars.rv)
            } else if cmd.len() >= 3 {
                match cmd[2].trim() {
                    "prev" => vars.rv = stack[stack.len() - 1],
                    _ => vars.rv = errors::parse_float(cmd[2], b, l),
                }
            } else {
                errors::args_error(b, l);
            }
        }
        "string" => {
            if cmd.len() < 4 {
                errors::args_error(b, l);
            }
            if cmd[2] != ">>" {
                // TODO: error statement
                errors::args_error(b, l);
            }
            let lits: Vec<&str> = statement.split(">>").collect();
            vars.string = lits[1].trim().to_string();
        }

        "lxstring" => {
            if cmd.len() < 4 {
                errors::args_error(b, l);
            }
            if cmd[2] != ">>" {
                // TODO: error statement
                errors::args_error(b, l);
            }
            let lits: Vec<&str> = statement.split(">>").collect();
            vars.lxstring = lits[1].trim().to_string();
        }
        "vec" => {
            if cmd.len() < 5 {
                errors::args_error(b, l)
            }
            if cmd[3] != ">>" {
                // TODO: error statement
                errors::args_error(b, l);
            }

            match cmd[2] {
                "str" => {
                    let lits: Vec<&str> = statement.split(">>").collect();
                    let data_vec: String = lits[1].trim().to_string();
                    let slice = &data_vec[1..data_vec.len() - 1];
                    let str_vec: Vec<&str> = slice.split(",").collect();
                    for item in str_vec {
                        vars.str_vec.push(item.trim().to_string());
                    }
                }
                "int" => {
                    let lits: Vec<&str> = statement.split(">>").collect();
                    let str_vec: String = lits[1].trim().to_string();
                    let slice = &str_vec[1..str_vec.len() - 1];
                    let data_vec: Vec<&str> = slice.split(",").collect();
                    for item in data_vec {
                        vars.num_vec.push(errors::parse_float(item, b, l));
                    }
                }
                _ => errors::args_error(b, l),
            }
        }

        "var" => {
            if cmd.len() < 3 {
                errors::args_error(b, l);
            }
            if cmd.len() == 3 {
                match hash_vars.hash_int.get(cmd[2].trim()) {
                    Some(&value) => stack.push(value),
                    _ => super::errors::var_error(cmd[2].trim(), b, l),
                }
            } else {
                if cmd[3].trim() == "prev" {
                    hash_vars
                        .hash_int
                        .insert(cmd[2].trim().to_string(), stack[stack.len() - 1]);
                }
            }
        }
        _ => stack.push(errors::parse_float(cmd[1], b, l)),
    }
}

pub fn pop(stack: &mut Vec<f64>, b: usize, l: u32) {
    if stack.len() < 1 {
        super::errors::stack_len_error(b, l);
    }
    stack.pop();
}

pub fn strfn(
    stack: &mut Vec<f64>,
    vars: &mut super::super::Vars,
    cmd: Vec<&str>,
    b: usize,
    l: u32,
) {
    if cmd.len() < 2 {
        super::errors::args_error(b, l);
    }
    match cmd[1].trim() {
        "cmp" => {
            if vars.lxstring.trim() == vars.string.trim() {
                stack.push(0.0);
            } else {
                stack.push(-1.0);
            }
        }
        "string" => vars.string = vars.lxstring.clone(),
        "lxstring" => vars.string = vars.lxstring.clone(),
        _=> errors::args_error(b, l),
    }
}
