use super::errors::ErrorKind;
use crate::{memory::Memory, types::{Vars, Type, TypeName}, CPU};

pub fn map(
    memory: &mut Memory,
    vars: &mut Vars,
    registers: &mut CPU,
    cmd: Vec<&str>,
    statement: &str,
) -> Result<(), ErrorKind> {
    if cmd.len() < 3 {
        return Err(ErrorKind::ArgErr);
    }
    match cmd[0] {
        "ram" => vars.add_map(cmd[1].to_string()),
        "get" => {
            let t: Type = vars.get_from_map(cmd[1].to_string(), cmd[2].to_string(), memory)?;
            let assign_var = vars.get_type(cmd[4].to_string())?;
            match t.name {
                TypeName::String => {
                    let old_str_addr = memory.load(assign_var.location).to_owned();
                    let t_addr = memory.load(t.location).to_owned();
                    memory.stack_mod(assign_var.location, &t_addr);
                    memory.free(u32::from_be_bytes(old_str_addr.try_into().unwrap()));
                },
                TypeName::I32 => {
                    let t_data = memory.load(t.location).to_owned();
                    memory.stack_mod(assign_var.location, &t_data);
                },
                _ => unimplemented!()
            }
        },
        "insert" => {
            let kv = Kv::new(statement)?;
            vars.insert_to_map(cmd[1].to_string(), kv.key, kv.value, memory)?
        }
        "delete" => vars.remove_from_map(cmd[1].to_string(), cmd[2].to_string(), memory)?,
        _ => return Err(ErrorKind::ArgErr),
    }
    Ok(())
}

struct Kv {
    key: String,
    value: String,
}

impl Kv {
    pub fn new(statement: &str) -> Result<Self, ErrorKind>{
        let key = statement.split(':').collect::<Vec<&str>>()[0]
            .split('{')
            .collect::<Vec<&str>>()[1]
            .trim();
        
        let value = statement.split(':').collect::<Vec<&str>>()[1]
            .split('}')
            .collect::<Vec<&str>>()[0]
            .trim();

        if value.len() < 1 || key.len() < 1 {
            return Err(ErrorKind::ArgErr)
        }

        Ok(Self { key: key.to_string(), value: value.to_string() })
    }
}
