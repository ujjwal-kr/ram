use crate::memory::Memory;
use crate::types::{Type, TypeName, Vars};
use crate::CPU;

use super::errors::ErrorKind;

pub fn print(
    memory: &mut Memory,
    vars: &mut Vars,
    registers: &mut CPU,
    cmd: Vec<&str>,
) -> Result<(), ErrorKind> {
    // print var <var_name> { lx/rv/string/lxstring/var-name }
    // print hello world
    if cmd.len() == 1 {
        println!("{}", memory.get_int_from_stack()?);
        Ok(())
    } else if cmd[1] == "var" {
        match cmd[2] {
            "lx" => println!("{}", registers.lx),
            "rv" => println!("{}", registers.rv),
            "string" => println!("{}", registers.string),
            "lxstring" => println!("{}", registers.lxstring),
            _ => {
                if cmd.len() == 3 {
                    let _type: Type = vars.get_type(cmd[2].to_string())?;
                    match _type.name {
                        TypeName::I32 => println!("{}", memory.yeild_i32(_type.location)),
                        TypeName::String => println!("{}", memory.yeild_string(_type.location)),
                        TypeName::Vector => todo!("Need to identify vector types"),
                        _ => todo!("map for other types"),
                    }
                } else {
                    let print_st = &cmd[1..cmd.len()].to_vec().join(" ").to_string();
                    println!("{print_st}");
                }
            }
        }
        Ok(())
    } else {
        let print_st = &cmd[1..cmd.len()].to_vec().join(" ").to_string();
        println!("{print_st}");
        Ok(())
    }
}
