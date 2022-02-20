use super::super::errors;
use super::super::Vars;

pub fn var<'a>(cmd: Vec<&'a str>, statement: &'a str, vars: &mut Vars<'a>, b: usize, l: u32) {
    if cmd.len() < 5 {
        errors::args_error(b, l);
    }
    let lits: Vec<&str> = statement.split(">>").collect();
    let value: &str = lits[1].trim();

    if cmd[2] == "str" {
        vars.hash_str.insert(cmd[1].trim(), value);
    } else if cmd[2] == "int" {
        vars.hash_int
            .insert(cmd[1].trim(), errors::parse_float(value, b, l));
    } else {
        errors::args_error(b, l);
    }
}

pub fn movefn(cmd: Vec<&str>, vars: &mut Vars, b: usize, l: u32) {
    if cmd.len() < 4 {
        errors::args_error(b, l);
    }
    if cmd[1] == "str" {
        if cmd[2] == "string" {
            match vars.hash_str.get(&cmd[3].trim()) {
                Some(&value) => { vars.string = value.to_string(); },
                _ => { errors::var_error(cmd[3], b, l); }
            }
        } else if cmd[2] == "lxstring" {
            match vars.hash_str.get(&cmd[3].trim()) {
                Some(&value) => { vars.lxstring = value.to_string() },
                _ => { errors::var_error(cmd[3], b, l); }
            }
        } else if cmd[2] == "var" {
            
        } else {
            errors::args_error(b, l);
        }
    } else if cmd[1] == "int" {
        if cmd[2] == "lx" {
            match vars.hash_int.get(&cmd[3].trim()) {
                Some(&value) => { vars.lx = value },
                _ => { errors::var_error(cmd[3], b, l) }
            }
        } else if cmd[2] == "rv" {
            match vars.hash_int.get(&cmd[3].trim()) {
                Some(&value) => { vars.rv = value },
                _ => { errors::var_error(cmd[3], b, l) }
            }
        } else if cmd[2] == "var" {
            
        } else {
            errors::args_error(b, l);
        }
    }
}
