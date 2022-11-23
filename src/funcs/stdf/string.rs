use crate::funcs::errors::ErrorKind;
use crate::types::{TypeName, Vars};
use crate::{memory::Memory, CPU};

pub fn split(
    memory: &mut Memory,
    vars: &mut Vars,
    registers: &mut CPU,
    cmd: Vec<&str>,
    statement: &str,
) -> Result<(), ErrorKind> {
    let var_str = statement.split('=').collect::<Vec<&str>>()[1].trim();
    let t = vars.get_type(var_str.to_string())?;
    if t.name != TypeName::Vector {
        return Err(ErrorKind::ExpectedVec(var_str.to_string()));
    }
    let del_str = statement.split('>').collect::<Vec<&str>>()[1];
    let final_str = del_str.split('=').collect::<Vec<&str>>()[0].trim();
    let delimiter = &final_str[1..final_str.len() - 1];

    match cmd[1] {
        "string" => vars.set_raw_str_vec(
            var_str.to_string(),
            registers.string.split(delimiter).collect::<Vec<&str>>(),
            memory,
        ),
        "lxstring" => vars.set_raw_str_vec(
            var_str.to_string(),
            registers.lxstring.split(delimiter).collect::<Vec<&str>>(),
            memory,
        ),
        _ => {
            let split_var_str = cmd[1];
            let split_var_str_type = vars.get_type(split_var_str.to_string())?;
            if split_var_str_type.name == TypeName::String {
                let split_var = memory.yeild_string(split_var_str_type.location);
                vars.set_raw_str_vec(
                    var_str.to_string(),
                    split_var.split(delimiter).collect::<Vec<&str>>(),
                    memory,
                );
            } else {
                return Err(ErrorKind::ExpectedStr(split_var_str.to_string()));
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
