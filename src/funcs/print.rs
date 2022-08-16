pub fn print(
    stack: &mut Vec<u8>,
    cmd: Vec<&str>,
    vars: &mut super::super::Vars,
    hash_vars: &mut super::super::HashVars,
    b: &str,
    l: u32,
) {
    if cmd.len() == 1 {
        if stack.is_empty() {
            super::errors::stack_len_error(b, l);
        } else {
            println!("{}", stack[stack.len() - 1]);
        }
    } else {
        match cmd[1].trim() {
            "lx" => println!("{}", vars.lx),
            "rv" => println!("{}", vars.rv),
            "string" => println!("{}", vars.string.trim()),
            "lxstring" => println!("{}", vars.lxstring.trim()),
            "vec" => match cmd[2].trim() {
                "str" => {
                    if cmd.len() == 3 {
                        println!("{:?}", vars.str_vec);
                    }
                }
                "int" => {
                    if cmd.len() == 3 {
                        println!("{:?}", vars.num_vec);
                    }
                }
                _ => super::errors::args_error(b, l),
            },
            "global_var" => match cmd[3].trim() {
                "int" => {
                    if cmd.len() == 5 && cmd[4] == "vec" {
                        match hash_vars.hash_int_vec.get(cmd[2].trim()) {
                            Some(value) => println!("{:?}", value),
                            _ => super::errors::var_error(cmd[2].trim(), b, l),
                        }
                    } else {
                        match hash_vars.hash_int.get(cmd[2].trim()) {
                            Some(&value) => println!("{}", value),
                            _ => super::errors::var_error(cmd[2].trim(), b, l),
                        }
                    }
                }
                "str" => {
                    if cmd.len() == 5 && cmd[4] == "vec" {
                        match hash_vars.hash_str_vec.get(cmd[2].trim()) {
                            Some(value) => println!("{:?}", value),
                            _ => super::errors::var_error(cmd[2], b, l),
                        }
                    } else {
                        match hash_vars.hash_str.get(cmd[2].trim()) {
                            Some(value) => println!("{}", value),
                            _ => super::errors::var_error(cmd[2].trim(), b, l),
                        }
                    }
                }
                _ => super::errors::args_error(b, l),
            },
            "var" => match cmd[3].trim() {
                "int" => {
                    if cmd.len() == 5 && cmd[4] == "vec" {
                        match vars.var_int_vec.get(cmd[2].trim()) {
                            Some(value) => println!("{:?}", value),
                            _ => super::errors::var_error(cmd[2].trim(), b, l),
                        }
                    } else {
                        match vars.var_int.get(cmd[2].trim()) {
                            Some(&value) => println!("{}", value),
                            _ => super::errors::var_error(cmd[2].trim(), b, l),
                        }
                    }
                }
                "str" => {
                    if cmd.len() == 5 && cmd[4] == "vec" {
                        match vars.var_str_vec.get(cmd[2].trim()) {
                            Some(value) => println!("{:?}", value),
                            _ => super::errors::var_error(cmd[2], b, l),
                        }
                    } else {
                        match vars.var_str.get(cmd[2].trim()) {
                            Some(value) => println!("{}", value),
                            _ => super::errors::var_error(cmd[2].trim(), b, l),
                        }
                    }
                }
                _ => super::errors::args_error(b, l),
            },
            _ => super::errors::args_error(b, l),
        }
    }
}

pub fn printc(cmd: Vec<&str>, statement: &str, b: &str, l: u32) {
    if cmd.len() < 3 {
        super::errors::args_error(b, l);
    } else {
        let prntc_cmd: Vec<&str> = statement.split(">>").collect();
        println!("{}", prntc_cmd[1].trim());
    }
}
