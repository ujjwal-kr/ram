use crate::funcs::errors::ErrorKind;
use crate::types::{TypeName, Vars, Vector};
use crate::{memory::Memory, CPU};

pub fn split(
    memory: &mut Memory,
    vars: &mut Vars,
    registers: &mut CPU,
    cmd: Vec<&str>,
    statement: &str,
) -> Result<(), ErrorKind> {
    if cmd.len() < 6 {
        return Err(ErrorKind::ArgErr);
    }
    let var_str = cmd[cmd.len() - 1].trim();
    let t = vars.get_type(var_str.to_string())?;
    if t.name != TypeName::Vector(Vector::String) {
        return Err(ErrorKind::ExpectedVec(var_str.to_string()));
    }
    let mut qc = 0i32;
    let mut delimiter = String::from("");
    for c in statement.chars() {
        if c == '"' {
            qc += 1;
        }
        if qc == 1 && c != '"' {
            delimiter.push(c)
        }
    }
    let binding = append_escapes(&delimiter);
    delimiter = binding;
    match cmd[1] {
        "string" => vars.set_raw_str_vec(
            var_str.to_string(),
            registers
                .string
                .split(delimiter.as_str())
                .filter(|&x| x != delimiter)
                .collect::<Vec<&str>>(),
            memory,
        ),
        "lxstring" => vars.set_raw_str_vec(
            var_str.to_string(),
            registers
                .lxstring
                .split(delimiter.as_str())
                .filter(|&x| x != delimiter)
                .collect::<Vec<&str>>(),
            memory,
        ),
        _ => {
            let split_var_str = cmd[1];
            let split_var_str_type = vars.get_type(split_var_str.to_string())?;
            if split_var_str_type.name == TypeName::String {
                let split_var = memory.yeild_string(split_var_str_type.location);
                vars.set_raw_str_vec(
                    var_str.to_string(),
                    split_var
                        .split(delimiter.as_str())
                        .filter(|&x| x != delimiter)
                        .collect::<Vec<&str>>(),
                    memory,
                );
            } else {
                return Err(ErrorKind::ExpectedStr(split_var_str.to_string()));
            }
        }
    }
    Ok(())
}

fn append_escapes(del: &str) -> String {
    let mut x = del.replacen(r"\n", "\n", del.len());
    x = x.replacen(r"\r", "\r", del.len());
    x = x.replacen(r"\t", "\t", del.len());
    x
}

pub fn join(
    memory: &mut Memory,
    vars: &mut Vars,
    registers: &mut CPU,
    cmd: Vec<&str>,
    statement: &str,
) -> Result<(), ErrorKind> {
    if cmd.len() < 6 {
        return Err(ErrorKind::ArgErr);
    }
    let assign_to = cmd[cmd.len() - 1].to_string();
    let vec_str = cmd[1].to_string();
    let del_exp = statement.split('=').collect::<Vec<&str>>()[0]
        .trim()
        .split('>')
        .collect::<Vec<&str>>()[1]
        .trim();
    let delimiter: &str = &del_exp[1..del_exp.len() - 1];
    let var = vars.get_type(vec_str.clone())?;
    if var.name != TypeName::Vector(Vector::String) {
        return Err(ErrorKind::ExpectedStr(vec_str));
    }
    match assign_to.trim() {
        "string" => registers.string = memory.yeild_str_vec(var.location).join(delimiter),
        "lxstring" => registers.string = memory.yeild_str_vec(var.location).join(delimiter),
        _ => {
            let t = vars.get_type(assign_to.clone())?;
            if t.name == TypeName::String {
                let heap_addr = u32::from_be_bytes(memory.load(t.location).try_into().unwrap());
                let vec = memory.yeild_str_vec(var.location);
                memory.heap_mod(heap_addr, vec.join(delimiter).as_bytes())
            } else {
                return Err(ErrorKind::ExpectedStr(assign_to));
            }
        }
    }
    Ok(())
}

pub fn concat(
    memory: &mut Memory,
    vars: &mut Vars,
    registers: &mut CPU,
    cmd: Vec<&str>,
) -> Result<(), ErrorKind> {
    if cmd.len() > 3 {
        return Err(ErrorKind::ArgErr);
    }

    match cmd[1] {
        "string" => match cmd[2] {
            "string" => registers.string = format!("{}{}", registers.string, registers.string),
            "lxstring" => registers.string = format!("{}{}", registers.string, registers.lxstring),
            _ => {
                let t = vars.get_type(cmd[2].to_string())?;
                if t.name == TypeName::String {
                    registers.string =
                        format!("{}{}", registers.string, memory.yeild_string(t.location))
                } else {
                    return Err(ErrorKind::ExpectedStr(cmd[2].to_string()));
                }
            }
        },
        "lxstring" => match cmd[2] {
            "string" => registers.string = format!("{}{}", registers.lxstring, registers.string),
            "lxstring" => {
                registers.string = format!("{}{}", registers.lxstring, registers.lxstring)
            }
            _ => {
                let t = vars.get_type(cmd[2].to_string())?;
                if t.name == TypeName::String {
                    registers.string =
                        format!("{}{}", registers.lxstring, memory.yeild_string(t.location))
                } else {
                    return Err(ErrorKind::ExpectedStr(cmd[2].to_string()));
                }
            }
        },
        _ => {
            let t = vars.get_type(cmd[1].to_string())?;
            match cmd[2] {
                "string" => {
                    registers.string =
                        format!("{}{}", memory.yeild_string(t.location), registers.string)
                }
                "lxstring" => {
                    registers.string =
                        format!("{}{}", memory.yeild_string(t.location), registers.lxstring)
                }
                _ => {
                    let t2 = vars.get_type(cmd[2].to_string())?;
                    registers.string = format!(
                        "{}{}",
                        memory.yeild_string(t.location),
                        memory.yeild_string(t2.location)
                    )
                }
            }
        }
    }
    Ok(())
}

pub fn trim(
    memory: &mut Memory,
    vars: &mut Vars,
    registers: &mut CPU,
    cmd: Vec<&str>,
) -> Result<(), ErrorKind> {
    if cmd.len() != 2 {
        return Err(ErrorKind::ArgErr);
    }
    match cmd[1] {
        "string" => registers.string = registers.string.trim().to_string(),
        "lxstring" => registers.lxstring = registers.lxstring.trim().to_string(),
        _ => {
            let t = vars.get_type(cmd[1].to_string())?;
            if t.name == TypeName::String {
                let value = memory.yeild_string(t.location);
                vars.set_string(cmd[1].to_string(), value.trim(), memory);
            } else {
                return Err(ErrorKind::ExpectedStr(cmd[1].to_string()));
            }
        }
    }
    Ok(())
}
