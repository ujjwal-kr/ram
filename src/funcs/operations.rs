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
    if cmd[1] == "lx" || cmd[1] == "rv" {
        if cmd[1] == "lx" {
            vars.lx = vars.lx * vars.lx;
        }
        if cmd[1] == "rv" {
            vars.rv = vars.rv * vars.rv;
        }
    } else {
        if stack.len() < 1 {
            super::errors::stack_len_error(b, l);
        }
        let result = stack[stack.len() - 1] * stack[stack.len() - 1];
        stack.push(result);
    }
}

pub fn sqrt(stack: &mut Vec<f64>, cmd: Vec<&str>, vars: &mut super::super::Vars, b: usize, l: u32) {
    if cmd[1] == "lx" || cmd[1] == "rv" {
        if cmd[1] == "lx" {
            vars.lx = vars.lx.sqrt();
        }
        if cmd[1] == "rv" {
            vars.rv = vars.rv.sqrt();
        }
    } else {
        let result = stack[stack.len() - 1].sqrt();
        stack.push(result);
    }
    if stack.len() < 1 {
        super::errors::stack_len_error(b, l);
    }
    let result = stack[stack.len() - 1].sqrt();
    stack.push(result);
}

pub fn round(
    stack: &mut Vec<f64>,
    cmd: Vec<&str>,
    vars: &mut super::super::Vars,
    b: usize,
    l: u32,
) {
    if cmd.len() > 1 {
        if cmd[1] == "lx" || cmd[1] == "rv" {
            if cmd[1] == "lx" {
                vars.lx = vars.lx.round();
            }
            if cmd[1] == "rv" {
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

pub fn split(cmd: Vec<&str>, statement: &str, vars: &mut super::super::Vars, b: usize, l: u32) {
    if cmd.len() < 3 {
        super::errors::args_error(b, l)
    } else {
        let args: Vec<&str> = statement.split(">>").collect();
        let string: &str = vars.string.trim();
        let str_vec: Vec<&str> = string.split(args[1].trim()).collect();
        for items in str_vec {
          vars.str_vec.push(items.trim().to_string());
        }
    }
}
