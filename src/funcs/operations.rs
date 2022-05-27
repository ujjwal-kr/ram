pub fn add(stack: &mut Vec<f64>, cmd: Vec<&str>, vars: &mut super::super::Vars, b: usize, l: u32) {
    if cmd.len() > 1 {
        stack.push(vars.lx + vars.rv);
    } else {
        if stack.len() < 2 {
            super::errors::stack_len_error(b, l);
        } else {
            let result = stack[stack.len() - 1] + stack[stack.len() - 2];
            stack.push(result);
        }
    }
}

pub fn sub(stack: &mut Vec<f64>, b: usize, l: u32) {
    if stack.len() < 2 {
        super::errors::stack_len_error(b, l);
    }
    let result = stack[stack.len() - 2] - stack[stack.len() - 1];
    stack.push(result);
}

pub fn mul(stack: &mut Vec<f64>, cmd: Vec<&str>, vars: &mut super::super::Vars, b: usize, l: u32) {
    if cmd.len() > 1 {
        stack.push(vars.lx * vars.rv);
    } else {
        if stack.len() < 2 {
            super::errors::stack_len_error(b, l);
        }
        let result = stack[stack.len() - 1] * stack[stack.len() - 2];
        stack.push(result);
    }
}

pub fn div(stack: &mut Vec<f64>, b: usize, l: u32) {
    if stack.len() < 2 {
        super::errors::stack_len_error(b, l);
    }
    let result = stack[stack.len() - 2] / stack[stack.len() - 1];
    stack.push(result)
}

pub fn sqr(stack: &mut Vec<f64>, cmd: Vec<&str>, vars: &mut super::super::Vars, b: usize, l: u32) {
    if cmd.len() == 1 {
        if stack.len() < 1 {
            super::errors::stack_len_error(b, l);
        }
        let result = stack[stack.len() - 1] * stack[stack.len() - 1];
        stack.push(result);
    } else {
        if cmd[1].trim() == "lx" || cmd[1].trim() == "rv" {
            if cmd[1].trim() == "lx" {
                vars.lx = vars.lx * vars.lx;
            }
            if cmd[1].trim() == "rv" {
                vars.rv = vars.rv * vars.rv;
            }
        } else {
            super::errors::args_error(b, l);
        }
    }
}

pub fn sqrt(stack: &mut Vec<f64>, cmd: Vec<&str>, vars: &mut super::super::Vars, b: usize, l: u32) {
    if cmd.len() == 1 {
        if stack.len() < 1 {
            super::errors::stack_len_error(b, l);
        } else {
            let result = stack[stack.len() - 1].sqrt();
            stack.push(result);
        }
    } else {
        if cmd[1] == "lx" || cmd[1] == "rv" {
            if cmd[1].trim() == "lx" {
                vars.lx = vars.lx.sqrt();
            }
            if cmd[1].trim() == "rv" {
                vars.rv = vars.rv.sqrt();
            }
        } else {
            super::errors::args_error(b, l);
        }
    }
}

pub fn round(
    stack: &mut Vec<f64>,
    cmd: Vec<&str>,
    vars: &mut super::super::Vars,
    b: usize,
    l: u32,
) {
    if cmd.len() > 1 {
        if cmd[1].trim() == "lx" || cmd[1].trim() == "rv" {
            if cmd[1].trim() == "lx" {
                vars.lx = vars.lx.round();
            }
            if cmd[1].trim() == "rv" {
                vars.rv = vars.rv.round();
            }
        }
    } else {
        if stack.len() < 1 {
            super::errors::stack_len_error(b, l);
        }
        let result = stack[stack.len() - 1].round();
        stack.push(result);
    }
}

pub fn avg(stack: &mut Vec<f64>, b: usize, l: u32) {
    if stack.len() < 1 {
        super::errors::stack_len_error(b, l);
    }
    let mut total: f64 = 0.0;
    let mut i: f64 = 0.0;
    for num in stack.clone() {
        i = i + 1.0;
        total = total + num
    }
    stack.push(total / i)
}

pub fn cmp(stack: &mut Vec<f64>, b: usize, l: u32) {
    if stack.len() < 2 {
        super::errors::stack_len_error(b, l);
    }
    if stack[stack.len() - 1] < stack[stack.len() - 2] {
        stack.push(1.0)
    } else if stack[stack.len() - 1] > stack[stack.len() - 2] {
        stack.push(-1.0)
    } else {
        stack.push(0.0)
    }
}

// strings

pub fn split(cmd: Vec<&str>, statement: &str, vars: &mut super::super::Vars, b: usize, l: u32) {
    if cmd.len() < 3 {
        super::errors::args_error(b, l);
    } else {
        let lits: Vec<&str> = statement.split(">>").collect();
        let string: &str = vars.string.trim();
        let str_arg: &str = lits[1].trim();
        let slice = &str_arg[1..str_arg.len() - 1];
        if slice == r"\n" || slice == r"\n\n" {
            if slice == r"\n" {
                let str_vec: Vec<&str> = string.split("\n").collect();
                vars.str_vec = vec![];
                for items in str_vec {
                    vars.str_vec.push(items.to_string());
                }
            } else if slice == r"\n\n" {
                let str_vec: Vec<&str> = string.split("\n\n").collect();
                vars.str_vec = vec![];
                for items in str_vec {
                    vars.str_vec.push(items.to_string());
                }
            } else if slice == r"\t" {
                let str_vec: Vec<&str> = string.split("\t").collect();
                vars.str_vec = vec![];
                for items in str_vec {
                    vars.str_vec.push(items.to_string());
                }
            }
        } else {
            let str_vec: Vec<&str> = string.split(slice).collect();
            vars.str_vec = vec![];
            for items in str_vec {
                vars.str_vec.push(items.to_string());
            }
        }
    }
}

// vectors

pub fn vec_ops(
    stack: &mut Vec<f64>,
    cmd: Vec<&str>,
    statement: &str,
    vars: &mut super::super::Vars,
    b: usize,
    l: u32,
) {
    if cmd.len() < 3 {
        super::errors::args_error(b, l);
    }
    match cmd[1] {
        "str" => match cmd[2].trim() {
            "push" => vars.str_vec.push(vars.string.clone().trim().to_string()),
            "pop" => {
                if vars.str_vec.is_empty() {
                    super::errors::vec_items(b, l);
                } else {
                    vars.str_vec.pop();
                }
            }
            "len" => {
                stack.push(super::errors::parse_float(
                    vars.str_vec.len().to_string().trim(),
                    b,
                    l,
                ));
            }
            ">>" => {
                let lits: Vec<&str> = statement.split(">>").collect();
                let data_vec = lits[1].trim();
                let slice = &data_vec[1..data_vec.len() - 1];
                let index: usize = ret_index(slice.trim(), vars, b, l);
                if index >= vars.str_vec.len() {
                    super::errors::invalid_index(b, l, index);
                }
                vars.string = vars.str_vec[index].to_string();
            }
            _ => super::errors::args_error(b, l),
        },
        "int" => match cmd[2].trim() {
            "push" => match cmd[3].trim() {
                "lx" => vars.num_vec.push(vars.lx),
                "rv" => vars.num_vec.push(vars.rv),
                _ => super::errors::args_error(b, l),
            },
            "pop" => {
                if vars.num_vec.is_empty() {
                    super::errors::vec_items(b, l);
                }
                vars.num_vec.pop();
            }
            "len" => {
                stack.push(super::errors::parse_float(
                    vars.num_vec.len().to_string().trim(),
                    b,
                    l,
                ));
            }
            "lx" | "rv" => {
                if cmd[3] == ">>" {
                    let lits: Vec<&str> = statement.split(">>").collect();
                    let data_vec = lits[1].trim();
                    let slice = &data_vec[1..data_vec.len() - 1];
                    let index: usize = ret_index(slice.trim(), vars, b, l);
                    if index >= vars.num_vec.len() {
                        super::errors::invalid_index(b, l, index);
                    }
                    if cmd[2].trim() == "lx" {
                        vars.lx = vars.num_vec[index];
                    } else if cmd[2].trim() == "rv" {
                        vars.rv = vars.num_vec[index]
                    } else {
                        super::errors::args_error(b, l);
                    }
                } else {
                    super::errors::args_error(b, l);
                }
            }
            _ => super::errors::args_error(b, l),
        },
        _ => super::errors::args_error(b, l),
    }
}

fn ret_index(str_idx: &str, vars: &mut super::super::Vars, b: usize, l: u32) -> usize {
    if str_idx == "lx" || str_idx == "rv" {
        if str_idx == "lx" {
            let index: usize = super::errors::parse_usize(vars.lx.to_string().trim(), b, l);
            index
        } else {
            let index: usize = super::errors::parse_usize(vars.rv.to_string().trim(), b, l);
            index
        }
    } else {
        let index: usize = super::errors::parse_usize(str_idx, b, l);
        index
    }
}
