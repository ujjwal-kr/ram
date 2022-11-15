use crate::funcs::errors;
use crate::types::{TypeName, Vars};
use crate::{memory::Memory, Registers};

pub fn split(
    memory: &mut Memory,
    vars: &mut Vars,
    registers: &mut Registers,
    cmd: Vec<&str>,
    statement: &str,
    b: &str,
    l: i32,
) {
    let var_str = statement.split('=').collect::<Vec<&str>>()[1].trim();
    let t = vars.get_type(var_str.to_string(), b, l);
    if t.name != TypeName::Vector {
        panic!("Expected {} to be string vec at {}{}", var_str, b, l);
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
            let split_var_str_type = vars.get_type(split_var_str.to_string(), b, l);
            if split_var_str_type.name == TypeName::String {
                let split_var = memory.yeild_string(split_var_str_type.location);
                vars.set_raw_str_vec(
                    var_str.to_string(),
                    split_var.split(delimiter).collect::<Vec<&str>>(),
                    memory,
                );
            } else {
                panic!("Expected {} to be string at {}{}", split_var_str, b, l);
            }
        }
    }
}

pub fn concat(
    memory: &mut Memory,
    vars: &mut Vars,
    registers: &mut Registers,
    cmd: Vec<&str>,
    b: &str,
    l: i32,
) {
    if cmd.len() > 3 {
        errors::args_error(b, l)
    }

    match cmd[1] {
        "string" => match cmd[2] {
            "string" => registers.string = format!("{}{}", registers.string, registers.string),
            "lxstring" => registers.string = format!("{}{}", registers.string, registers.lxstring),
            _ => {
                let t = vars.get_type(cmd[2].to_string(), b, l);
                if t.name == TypeName::String {
                    registers.string = format!("{}{}", registers.string, memory.yeild_string(t.location))
                } else {
                    panic!("Expected {} to be string at {}{}", cmd[2], b, l);
                }
            }
        },
        "lxstring" => match cmd[2] {
            "string" => registers.string = format!("{}{}", registers.lxstring, registers.string),
            "lxstring" => registers.string = format!("{}{}", registers.lxstring, registers.lxstring),
            _ => {
                let t = vars.get_type(cmd[2].to_string(), b, l);
                if t.name == TypeName::String {
                    registers.string = format!("{}{}", registers.lxstring, memory.yeild_string(t.location))
                } else {
                    panic!("Expected {} to be string at {}{}", cmd[2], b, l);
                }
            }
        },
        _ => {
            let t = vars.get_type(cmd[1].to_string(), b, l);
            match cmd[2] {
                "string" => registers.string = format!("{}{}", memory.yeild_string(t.location), registers.string),
                "lxstring" => registers.string = format!("{}{}", memory.yeild_string(t.location), registers.lxstring),
                _ => {
                    let t2 = vars.get_type(cmd[2].to_string(), b, l);
                    registers.string = format!("{}{}", memory.yeild_string(t.location), memory.yeild_string(t2.location))
                }
            }
        }
    }
}