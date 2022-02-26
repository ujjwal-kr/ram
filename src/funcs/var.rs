use super::super::errors;
use super::super::HashVars;
use super::super::Vars;

pub fn var(
    cmd: Vec<&str>,
    statement: &str,
    vars: &mut Vars,
    hash_vars: &mut HashVars,
    b: usize,
    l: u32,
) {
    if cmd.len() < 4 {
        errors::args_error(b, l);
    }

    if cmd[2] == "str" {
        if cmd[3].trim() == "vec" {
            if cmd[4].trim() == "push" {
                // var <name_vec> str vec push >> string/lxstring/var <name>
            } else if cmd[4] == ">>" {
                let lits: Vec<&str> = statement.split(">>").collect();
                let value: String = lits[1].trim().to_string();
                let slice = &value[1..value.len() - 1];
                let data_vec: Vec<&str> = slice.split(",").collect();
                let mut str_vec: Vec<String> = vec![];
                for item in data_vec {
                    str_vec.push(item.to_string());
                }
                hash_vars
                    .hash_str_vec
                    .insert(cmd[1].trim().to_string(), str_vec);
            } else {
                if cmd.len() < 6 {
                    errors::args_error(b, l);
                }
                // var <name_vec> str vec string/lxstring/var <name> >> [2]
            }
        } else {
            let lits: Vec<&str> = statement.split(">>").collect();
            let value: String = lits[1].trim().to_string();
            hash_vars.hash_str.insert(cmd[1].trim().to_string(), value);
        }
    } else if cmd[2] == "int" {
        if cmd[3].trim() == "vec" {
            if cmd[4].trim() == "push" {
                // var <name_vec> int vec push >> lx/rv/var <name>
            } else if cmd[4] == ">>" {
                let lits: Vec<&str> = statement.split(">>").collect();
                let value: String = lits[1].trim().to_string();
                let slice = &value[1..value.len() - 1];
                let data_vec: Vec<&str> = slice.split(",").collect();
                let mut num_vec: Vec<f64> = vec![];
                for item in data_vec {
                    num_vec.push(errors::parse_float(item, b, l));
                }
                hash_vars
                    .hash_int_vec
                    .insert(cmd[1].trim().to_string(), num_vec);
            } else {
                if cmd.len() < 6 {
                    errors::args_error(b, l);
                }
                // var <name_vec> int vec lx/rv/var <name> >> [2]
            }
        } else {
            let lits: Vec<&str> = statement.split(">>").collect();
            let value: String = lits[1].trim().to_string();
            hash_vars.hash_int.insert(
                cmd[1].trim().to_string(),
                errors::parse_float(value.trim(), b, l),
            );
        }
    } else {
        errors::args_error(b, l);
    }
}

pub fn movefn(cmd: Vec<&str>, vars: &mut Vars, hash_vars: &mut HashVars, b: usize, l: u32) {
    if cmd.len() < 4 {
        errors::args_error(b, l);
    }
    if cmd[1] == "str" {
        if cmd[2] == "string".trim() {
            match hash_vars.hash_str.get(cmd[3].trim()) {
                Some(value) => {
                    vars.string = value.to_string();
                }
                _ => {
                    errors::var_error(cmd[3].trim(), b, l);
                }
            }
        } else if cmd[2].trim() == "lxstring" {
            match hash_vars.hash_str.get(cmd[3].trim()) {
                Some(value) => vars.lxstring = value.to_string(),
                _ => {
                    errors::var_error(cmd[3].trim(), b, l);
                }
            }
        } else if cmd[2] == "var" {
            if cmd.len() < 5 {
                errors::args_error(b, l);
            }
            if cmd[4].trim() == "string" {
                hash_vars
                    .hash_str
                    .insert(cmd[3].trim().to_string(), vars.string.clone());
            } else if cmd[4].trim() == "lxstring" {
                hash_vars
                    .hash_str
                    .insert(cmd[3].trim().to_string(), vars.lxstring.clone());
            } else {
                errors::args_error(b, l);
            }
        } else {
            errors::args_error(b, l);
        }
    } else if cmd[1].trim() == "int" {
        if cmd[2] == "lx" {
            match hash_vars.hash_int.get(&cmd[3].trim().to_string()) {
                Some(&value) => vars.lx = value,
                _ => errors::var_error(cmd[3], b, l),
            }
        } else if cmd[2].trim() == "rv" {
            match hash_vars.hash_int.get(&cmd[3].trim().to_string()) {
                Some(&value) => vars.rv = value,
                _ => errors::var_error(cmd[3], b, l),
            }
        } else if cmd[2].trim() == "var" {
            if cmd[4].trim() == "lx" {
                hash_vars
                    .hash_int
                    .insert(cmd[3].trim().to_string(), vars.lx);
            } else if cmd[4].trim() == "rv" {
                hash_vars
                    .hash_int
                    .insert(cmd[3].trim().to_string(), vars.rv);
            }
        } else {
            errors::args_error(b, l);
        }
    } else if cmd[1] == "vec" {
        // move vec vec str test
        // move vec vec int test3

        // move vec var test2 vec str
        // move vec var test4 vec int

        if cmd.len() < 5 {
            errors::args_error(b, l);
        }

        if cmd[2] == "vec" {
            if cmd[3] == "str" {
                match hash_vars.hash_str_vec.get(cmd[4].trim()) {
                    Some(value) => vars.str_vec = value.to_vec(),
                    _ => errors::var_error(cmd[5].trim(), b, l),
                }
            } else if cmd[3] == "int" {
                match hash_vars.hash_int_vec.get(cmd[4].trim()) {
                    Some(value) => vars.num_vec = value.to_vec(),
                    _ => errors::var_error(cmd[5].trim(), b, l),
                }
            } else {
                errors::args_error(b, l);
            }
        } else if cmd[2] == "var" {
            if cmd[5].trim() == "str" {
                hash_vars
                    .hash_str_vec
                    .insert(cmd[3].trim().to_string(), vars.str_vec.clone());
            } else if cmd[5].trim() == "int" {
                hash_vars
                    .hash_int_vec
                    .insert(cmd[3].trim().to_string(), vars.num_vec.clone());
            } else {
                errors::args_error(b, l);
            }
        } else {
            errors::args_error(b, l);
        }
    } else {
        errors::args_error(b, l);
    }
}
