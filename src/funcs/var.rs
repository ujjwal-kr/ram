use super::super::errors;
use super::super::HashVars;
use super::super::Vars;

pub fn var(
    stack: &mut Vec<f64>,
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
            if cmd[4].trim() == "len" {
                match hash_vars.hash_str_vec.get(cmd[1].trim()) {
                    Some(value) => stack.push(errors::parse_float(value.to_vec().len().to_string().trim(), b, l)),
                    _ => errors::var_error(cmd[1].trim(), b, l),
                }
            } else if cmd[4].trim() == "push" {
                // var <name_vec> str vec push >> string/lxstring/var <name>
                if cmd[6] == "string".trim() || cmd[6] == "lxstring".trim() || cmd[6] == "var" {
                    let mut new_vec: Vec<String> = vec![];
                    match hash_vars.hash_str_vec.get(cmd[1]) {
                        Some(value) => new_vec = value.to_vec(),
                        _ => errors::var_error(cmd[1].trim(), b, l),
                    }
                    if cmd[6] == "string".trim() {
                        new_vec.push(vars.string.clone());
                        if let Some(x) = hash_vars.hash_str_vec.get_mut(cmd[1]) {
                            *x = new_vec;
                        }
                    } else if cmd[6] == "lxstring".trim() {
                        new_vec.push(vars.lxstring.clone());
                        if let Some(x) = hash_vars.hash_str_vec.get_mut(cmd[1]) {
                            *x = new_vec;
                        }
                    } else if cmd[6] == "var" {
                        let mut var_value: String = "".to_string();
                        match hash_vars.hash_str.get(cmd[7].trim()) {
                            Some(value) => var_value = value.to_string(),
                            _ => errors::var_error(cmd[7].trim(), b, l),
                        }
                        new_vec.push(var_value.clone());
                        if let Some(x) = hash_vars.hash_str_vec.get_mut(cmd[1]) {
                            *x = new_vec;
                        }
                    } else {
                        errors::args_error(b, l);
                    }
                } else {
                    errors::args_error(b, l);
                }
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
                if cmd[4] == "string" || cmd[4] == "lxstring" || cmd[4] == "var" {

                    let mut var_vec: Vec<String> = vec![];
                    match hash_vars.hash_str_vec.get(cmd[1]) {
                        Some(value) => var_vec = value.to_vec(),
                        _ => errors::var_error(cmd[1].trim(), b, l),
                    }

                    let lits: Vec<&str> = statement.split(">>").collect();
                    let data_vec = lits[1].trim();
                    let slice = &data_vec[1..data_vec.len() - 1];
                    let index: usize = ret_index(slice.trim(), vars, hash_vars, b, l);
                    if index >= var_vec.len() {
                        errors::invalid_index(b, l, index);
                    }

                    if cmd[4] == "string" {
                        vars.string = var_vec[index].clone();
                    } else if cmd[4] == "lxstring" {
                        vars.lxstring = var_vec[index].clone();
                    } else if cmd[4] == "var" {
                        hash_vars.hash_str.insert(cmd[5].to_string(), var_vec[index].clone());
                    }
                } else {
                    errors::args_error(b, l);
                }
            }
        } else {
            let lits: Vec<&str> = statement.split(">>").collect();
            let value: String = lits[1].trim().to_string();
            hash_vars.hash_str.insert(cmd[1].trim().to_string(), value);
        }
    } else if cmd[2] == "int" {
        if cmd[3].trim() == "vec" {
            if cmd[4].trim() == "len" {
                match hash_vars.hash_int_vec.get(cmd[1].trim()) {
                    Some(value) => stack.push(errors::parse_float(value.to_vec().len().to_string().trim(), b, l)),
                    _ => errors::var_error(cmd[1].trim(), b, l),
                }
            } else if cmd[4].trim() == "push" {
                // var <name_vec> int vec push >> lx/rv/var <name>
                if cmd[6].trim() == "lx" || cmd[6].trim() == "rv" || cmd[6] == "var" {
                    let mut new_vec: Vec<f64> = vec![];
                    match hash_vars.hash_int_vec.get(cmd[1]) {
                        Some(value) => new_vec = value.to_vec(),
                        _ => errors::var_error(cmd[1].trim(), b, l),
                    }
                    if cmd[6].trim() == "lx" {
                        new_vec.push(vars.lx.clone());
                        if let Some(x) = hash_vars.hash_int_vec.get_mut(cmd[1]) {
                            *x = new_vec;
                        }
                    } else if cmd[6].trim() == "rv" {
                        new_vec.push(vars.rv.clone());
                        if let Some(x) = hash_vars.hash_int_vec.get_mut(cmd[1]) {
                            *x = new_vec;
                        }
                    } else if cmd[6] == "var" {
                        let mut var_value: f64 = 0.0;
                        match hash_vars.hash_int.get(cmd[7].trim()) {
                            Some(&value) => var_value = value,
                            _ => errors::var_error(cmd[7].trim(), b, l),
                        }
                        new_vec.push(var_value.clone());
                        if let Some(x) = hash_vars.hash_int_vec.get_mut(cmd[1]) {
                            *x = new_vec;
                        }
                    } else {
                        errors::args_error(b, l);
                    }
                } else {
                    errors::args_error(b, l);
                }
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
                if cmd[4] == "lx" || cmd[4] == "rv" || cmd[4] == "var" {

                    let mut var_vec: Vec<f64> = vec![];
                    match hash_vars.hash_int_vec.get(cmd[1]) {
                        Some(value) => var_vec = value.to_vec(),
                        _ => errors::var_error(cmd[7].trim(), b, l),
                    }

                    let lits: Vec<&str> = statement.split(">>").collect();
                    let data_vec = lits[1].trim();
                    let slice = &data_vec[1..data_vec.len() - 1];
                    let index: usize = ret_index(slice.trim(), vars, hash_vars, b, l);
                    if index >= var_vec.len() {
                        errors::invalid_index(b, l, index);
                    }

                    if cmd[4] == "lx" {
                        vars.lx = var_vec[index].clone();
                    } else if cmd[4] == "rv" {
                        vars.rv = var_vec[index].clone();
                    } else if cmd[4] == "var" {
                        hash_vars.hash_int.insert(cmd[5].to_string(), var_vec[index].clone());
                    }
                } else {
                    errors::args_error(b, l);
                }
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

fn ret_index(str_idx: &str, vars: &mut Vars, hash_vars: &mut HashVars, b: usize, l: u32) -> usize {
    if str_idx.trim().len() == 1 {
        if str_idx == "lx" || str_idx == "rv" {
            if str_idx == "lx" {
                let index: usize = errors::parse_usize(vars.lx.to_string().trim(), b, l);
                return index;
            } else {
                let index: usize = errors::parse_usize(vars.rv.to_string().trim(), b, l);
                return index;
            } 
        } else {
            let index: usize = errors::parse_usize(str_idx, b, l);
            return index;
        }
    } else {
        let mut index_n: usize = 0;
        let str_idx_vec: Vec<&str> = str_idx.split(" ").collect();
        match hash_vars.hash_int.get(str_idx_vec[1].to_string().trim()) {
            Some(&value) => index_n = errors::parse_usize(value.to_string().trim(), b, l),
            _ => errors::var_error(str_idx_vec[1].trim(), b, l),
        }
        return index_n;
    }
}