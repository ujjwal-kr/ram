use std::fs::File;
use std::io::prelude::*;

use crate::funcs::errors::ErrorKind;
use crate::types::{TypeName, Vars};
use crate::{memory::Memory, CPU};

pub fn stdfs(
    memory: &mut Memory,
    vars: &mut Vars,
    registers: &mut CPU,
    cmd: Vec<&str>,
) -> Result<(), ErrorKind> {
    // PARSER CHANGE WHEN FS WRITE IS ADDED

    // stdfs open lxstring = string
    // stdfs open string = lxstring
    // stdfs open var_str = string/lxstring/var_str

    if cmd.len() < 5 {
        return Err(ErrorKind::ArgErr);
    }
    let filename: String;
    match cmd[2] {
        "string" => filename = registers.string.clone(),
        "lxstring" => filename = registers.lxstring.clone(),
        _ => {
            let t = vars.get_type(cmd[2].to_string())?;
            if t.name == TypeName::String {
                filename = memory.yeild_string(t.location);
            } else {
                return Err(ErrorKind::ExpectedStr(cmd[2].to_string()));
            }
        }
    }

    let mut file = File::open(filename.trim()).expect("Cannot open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading file to string");

    match cmd[cmd.len() - 1] {
        "string" => registers.string = contents,
        "lxstring" => registers.lxstring = contents,
        _ => {
            let t = vars.get_type(cmd[cmd.len() - 1].to_string())?;
            if t.name == TypeName::String {
                let heap_addr = u32::from_be_bytes(memory.load(t.location).try_into().unwrap());
                memory.heap_mod(heap_addr, contents.as_bytes())
            } else {
                return Err(ErrorKind::ExpectedStr(cmd[cmd.len() - 1].to_string()));
            }
        }
    }

    Ok(())
}
