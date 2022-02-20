pub fn ram(
    stack: &mut Vec<f64>,
    cmd: Vec<&str>,
    statement: &str,
    vars: &mut super::super::Vars,
    b: usize,
    l: u32,
) {
    if cmd[1] == "lx"
        || cmd[1] == "rv"
        || cmd[1] == "string"
        || cmd[1] == "vec"
        || cmd[1] == "lxstring"
    {
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
                vars.string = lits[1].trim().to_string();
            }
            if cmd[1] == "lxstring" {
                let lits: Vec<&str> = statement.split(">>").collect();
                vars.lxstring = lits[1].trim().to_string();
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

pub fn strfn(
    stack: &mut Vec<f64>,
    vars: &mut super::super::Vars,
    cmd: Vec<&str>,
    b: usize,
    l: u32,
) {
    if cmd.len() < 2 {
        super::errors::args_error(b, l);
    } else {
        if cmd[1] == "string" && cmd[2] == "lxstring" {
            vars.string = vars.lxstring.clone();
        } else if cmd[1] == "lxstring" && cmd[2] == "string" {
            vars.lxstring = vars.string.clone();
        } else if cmd[1] == "cmp" {
            if vars.lxstring.trim() == vars.string.trim() {
                stack.push(0.0);
            } else {
                stack.push(-1.0);
            }
        }
    }
}
