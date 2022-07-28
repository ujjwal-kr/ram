use super::super::errors;
use super::super::HashVars;
use super::super::Vars;

pub fn global_var(
    stack: &mut Vec<f64>,
    cmd: Vec<&str>,
    statement: &str,
    vars: &mut Vars,
    hash_vars: &mut HashVars,
    b: &str,
    l: u32,
) {
    if cmd.len() < 4 {
        errors::args_error(b, l)
    }

    match cmd[2] {
        "str" => match cmd[3] {
            ">>" => {
                let lits: Vec<&str> = statement.split(">>").collect();
                let value: String = lits[1].trim().to_string();
                hash_vars.hash_str.insert(cmd[1].trim().to_string(), value);
            }
            "vec" => match cmd[4].trim() {
                ">>" => {
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
                }
                "push" => {
                    let mut new_vec: Vec<String> = vec![];
                    match hash_vars.hash_str_vec.get(cmd[1]) {
                        Some(value) => new_vec = value.to_vec(),
                        _ => errors::var_error(cmd[1].trim(), b, l),
                    }
                    match cmd[6].trim() {
                        "string" => {
                            new_vec.push(vars.string.clone());
                            if let Some(x) = hash_vars.hash_str_vec.get_mut(cmd[1]) {
                                *x = new_vec;
                            }
                        }
                        "lxstring" => {
                            new_vec.push(vars.lxstring.clone());
                            if let Some(x) = hash_vars.hash_str_vec.get_mut(cmd[1]) {
                                *x = new_vec;
                            }
                        }

                        "var" => {
                            let mut var_value: String = "".to_string();
                            match hash_vars.hash_str.get(cmd[7].trim()) {
                                Some(value) => var_value = value.to_string(),
                                _ => errors::var_error(cmd[7].trim(), b, l),
                            }
                            new_vec.push(var_value);
                            if let Some(x) = hash_vars.hash_str_vec.get_mut(cmd[1]) {
                                *x = new_vec;
                            }
                        }
                        _ => errors::args_error(b, l),
                    }
                }
                "string" => {
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

                    vars.string = var_vec[index].clone();
                }
                "lxstring" => {
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

                    vars.lxstring = var_vec[index].clone();
                }
                "var" => {
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

                    hash_vars
                        .hash_str
                        .insert(cmd[5].to_string(), var_vec[index].clone());
                }
                "len" => match hash_vars.hash_str_vec.get(cmd[1].trim()) {
                    Some(value) => stack.push(errors::parse_float(
                        value.to_vec().len().to_string().trim(),
                        b,
                        l,
                    )),
                    _ => errors::var_error(cmd[1].trim(), b, l),
                },
                _ => errors::args_error(b, l),
            },
            _ => errors::args_error(b, l),
        },
        "int" => match cmd[3] {
            ">>" => {
                let lits: Vec<&str> = statement.split(">>").collect();
                let value: String = lits[1].trim().to_string();
                hash_vars.hash_int.insert(
                    cmd[1].trim().to_string(),
                    errors::parse_float(value.trim(), b, l),
                );
            }
            "vec" => match cmd[4].trim() {
                ">>" => {
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
                }
                "push" => {
                    let mut new_vec: Vec<f64> = vec![];
                    match hash_vars.hash_int_vec.get(cmd[1]) {
                        Some(value) => new_vec = value.to_vec(),
                        _ => errors::var_error(cmd[1].trim(), b, l),
                    }
                    match cmd[6].trim() {
                        "lx" => {
                            new_vec.push(vars.lx.clone());
                            if let Some(x) = hash_vars.hash_int_vec.get_mut(cmd[1]) {
                                *x = new_vec;
                            }
                        }
                        "rv" => {
                            new_vec.push(vars.rv);
                            if let Some(x) = hash_vars.hash_int_vec.get_mut(cmd[1]) {
                                *x = new_vec;
                            }
                        }
                        "var" => {
                            let mut var_value: f64 = 0.0;
                            match hash_vars.hash_int.get(cmd[7].trim()) {
                                Some(&value) => var_value = value,
                                _ => errors::var_error(cmd[7].trim(), b, l),
                            }
                            new_vec.push(var_value.clone());
                            if let Some(x) = hash_vars.hash_int_vec.get_mut(cmd[1]) {
                                *x = new_vec;
                            }
                        }
                        _ => errors::args_error(b, l),
                    }
                }
                "lx" => {
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

                    vars.lx = var_vec[index].clone();
                }
                "rv" => {
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

                    vars.rv = var_vec[index];
                }
                "var" => {
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

                    hash_vars
                        .hash_int
                        .insert(cmd[5].to_string(), var_vec[index]);
                }
                "len" => match hash_vars.hash_int_vec.get(cmd[1].trim()) {
                    Some(value) => stack.push(errors::parse_float(
                        value.to_vec().len().to_string().trim(),
                        b,
                        l,
                    )),
                    _ => errors::var_error(cmd[1].trim(), b, l),
                },
                _ => errors::args_error(b, l),
            },
            _ => errors::args_error(b, l),
        },
        _ => errors::args_error(b, l),
    }
}

pub fn movefn(cmd: Vec<&str>, vars: &mut Vars, hash_vars: &mut HashVars, b: &str, l: u32) {
    if cmd.len() < 6 {
        errors::args_error(b, l);
    }

    // move int lx/rv = var <name>
    //  move int var <name> = lx/rv

    // move str string/lxstring = var <name>
    // move str var <name> = string/lxstring

    // move vec vec str = var <name>
    // move vec vec int = var <name>
    // move vec var <name> = vec int
    // move vec var <name> = vec str

    match cmd[1] {
        "int" => match cmd[2] {
            "lx" | "rv" => {
                if cmd[3] != "=" || cmd[4] != "var" {
                    errors::args_error(b, l);
                }
                if cmd[2] == "lx" {
                    match hash_vars.hash_int.get(&cmd[5].trim().to_string()) {
                        Some(&value) => vars.lx = value,
                        _ => errors::var_error(cmd[5], b, l),
                    }
                } else {
                    match hash_vars.hash_int.get(&cmd[5].trim().to_string()) {
                        Some(&value) => vars.rv = value,
                        _ => errors::var_error(cmd[5], b, l),
                    }
                }
            }

            "var" => {
                if cmd[4] != "=" {
                    errors::args_error(b, l);
                }
                match cmd[5] {
                    "lx" => {
                        hash_vars
                            .hash_int
                            .insert(cmd[3].trim().to_string(), vars.lx);
                    }
                    "rv" => {
                        hash_vars
                            .hash_int
                            .insert(cmd[3].trim().to_string(), vars.rv);
                    }
                    _ => errors::args_error(b, l),
                }
            }
            _ => errors::args_error(b, l),
        },

        "str" => match cmd[2] {
            "string" | "lxstring" => {
                if cmd[3] != "=" || cmd[4] != "var" {
                    errors::args_error(b, l);
                }
                if cmd[2] == "string" {
                    match hash_vars.hash_str.get(cmd[5].trim()) {
                        Some(value) => {
                            vars.string = value.to_string();
                        }
                        _ => {
                            errors::var_error(cmd[5].trim(), b, l);
                        }
                    }
                } else {
                    match hash_vars.hash_str.get(cmd[5].trim()) {
                        Some(value) => vars.lxstring = value.to_string(),
                        _ => {
                            errors::var_error(cmd[5].trim(), b, l);
                        }
                    }
                }
            }
            "var" => {
                if cmd[4] != "=" {
                    errors::args_error(b, l);
                }
                match cmd[5] {
                    "lxstring" => {
                        hash_vars
                            .hash_str
                            .insert(cmd[3].trim().to_string(), vars.lxstring.clone());
                    }
                    "string" => {
                        hash_vars
                            .hash_str
                            .insert(cmd[3].trim().to_string(), vars.string.clone());
                    }
                    _ => errors::args_error(b, l),
                }
            }
            _ => errors::args_error(b, l),
        },
        "vec" => match cmd[2] {
            "vec" => match cmd[3] {
                "str" | "int" => {
                    if cmd[4] != "=" || cmd[5] != "var" {
                        errors::args_error(b, l);
                    }
                    if cmd[3] == "str" {
                        match hash_vars.hash_str_vec.get(cmd[6].trim()) {
                            Some(value) => vars.str_vec = value.to_vec(),
                            _ => errors::var_error(cmd[6].trim(), b, l),
                        }
                    } else {
                        match hash_vars.hash_int_vec.get(cmd[6].trim()) {
                            Some(value) => vars.num_vec = value.to_vec(),
                            _ => errors::var_error(cmd[6].trim(), b, l),
                        }
                    }
                }
                _ => errors::args_error(b, l),
            },
            "var" => match cmd[6] {
                "str" | "int" => {
                    if cmd[4] != "=" || cmd[5] != "vec" {
                        errors::args_error(b, l);
                    }

                    if cmd[6] == "str" {
                        hash_vars
                            .hash_str_vec
                            .insert(cmd[3].trim().to_string(), vars.str_vec.clone());
                    } else {
                        hash_vars
                            .hash_int_vec
                            .insert(cmd[3].trim().to_string(), vars.num_vec.clone());
                    }
                }
                _ => errors::args_error(b, l),
            },
            _ => errors::args_error(b, l),
        },
        _ => errors::args_error(b, l),
    }
}

fn ret_index(str_idx: &str, vars: &mut Vars, hash_vars: &mut HashVars, b: &str, l: u32) -> usize {
    let idx_tokens: Vec<&str> = str_idx.split(" ").collect();
    if idx_tokens.len() == 1 {
        if str_idx == "lx" || str_idx == "rv" {
            if str_idx == "lx" {
                let index: usize = errors::parse_usize(vars.lx.to_string().trim(), b, l);
                index
            } else {
                let index: usize = errors::parse_usize(vars.rv.to_string().trim(), b, l);
                index
            }
        } else {
            let index: usize = errors::parse_usize(str_idx, b, l);
            index
        }
    } else {
        let mut index_n: usize = 0;
        let str_idx_vec: Vec<&str> = str_idx.split(" ").collect();
        match hash_vars.hash_int.get(str_idx_vec[1].to_string().trim()) {
            Some(&value) => index_n = errors::parse_usize(value.to_string().trim(), b, l),
            _ => errors::var_error(str_idx_vec[1].trim(), b, l),
        }
        index_n
    }
}
