pub fn print(
    stack: &mut Vec<f64>,
    cmd: Vec<&str>,
    vars: &mut super::super::Vars,
    hash_vars: &mut super::super::HashVars,
    b: usize,
    l: u32,
) {
    if cmd.len() == 1 {
        if stack.len() < 1 {
            super::errors::stack_len_error(b, l);
        } else {
            println!("{}", stack[stack.len() - 1]);
        }
    } else {
        if cmd[1] == "lx" {
            println!("{}", vars.lx)
        } else if cmd[1].trim() == "rv" {
            println!("{}", vars.rv)
        } else if cmd[1].trim() == "string" {
            println!("{}", vars.string.trim());
        } else if cmd[1].trim() == "lxstring" {
            println!("{}", vars.lxstring.trim());
        } else if cmd[1] == "vec" {
            if cmd[2].trim() == "int" {
                if cmd.len() == 3 {
                    println!("{:?}", vars.num_vec);
                } else if cmd.len() == 4 {
                    match hash_vars.hash_int_vec.get(cmd[3].trim()) {
                        Some(value) => println!("{:?}", value),
                        _ => super::errors::var_error(cmd[3].trim(), b, l),
                    }
                }
            } else if cmd[2].trim() == "str" {
                if cmd.len() == 3 {
                    println!("{:?}", vars.str_vec);
                }
                if cmd.len() == 4 {
                    match hash_vars.hash_str_vec.get(cmd[3].trim()) {
                        Some(value) => println!("{:?}", value),
                        _ => super::errors::var_error(cmd[3], b, l),
                    }
                }
            } else {
                super::errors::var_error(cmd[2], b, l);
            }
        } else if cmd[1] == "var" {
            if cmd[2].trim() == "int" {
                match hash_vars.hash_int.get(cmd[3].trim()) {
                    Some(&value) => println!("{}", value),
                    _ => super::errors::var_error(cmd[3].trim(), b, l),
                }
            }
            else if cmd[2].trim() == "str" {
                match hash_vars.hash_str.get(cmd[3].trim()) {
                    Some(value) => println!("{}", value),
                    _ => super::errors::var_error(cmd[3].trim(), b, l),
                }
            }
            else { super::errors::args_error(b, l); }
        } else {
            super::errors::var_error(cmd[1], b, l);
        }
    }
}

pub fn printc(cmd: Vec<&str>, statement: &str, b: usize, l: u32) {
    if cmd.len() < 3 {
        super::errors::args_error(b, l);
    } else {
        let prntc_cmd: Vec<&str> = statement.split(">>").collect();
        println!("{}", prntc_cmd[1].trim());
    }
}
