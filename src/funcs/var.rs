use super::super::errors;
use super::super::HashVars;
use super::super::Vars;

pub fn var(cmd: Vec<&str>, statement: &str, hash_vars: &mut HashVars, b: usize, l: u32) {
    if cmd.len() < 5 {
        errors::args_error(b, l);
    }
    let lits: Vec<&str> = statement.split(">>").collect();
    let value: String = lits[1].trim().to_string();
    if cmd[2] == "str" {
        hash_vars.hash_str.insert(cmd[1].trim().to_string(), value);
    } else if cmd[2] == "int" {
        hash_vars.hash_int.insert(
            cmd[1].trim().to_string(),
            errors::parse_float(value.trim(), b, l),
        );
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
                hash_vars.hash_str
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
                hash_vars.hash_int.insert(cmd[3].trim().to_string(), vars.lx);
            } else if cmd[4].trim() == "rv" {
                hash_vars.hash_int.insert(cmd[3].trim().to_string(), vars.rv);
            }
        } else {
            errors::args_error(b, l);
        }
    }
}
