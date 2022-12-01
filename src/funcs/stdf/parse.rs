use crate::funcs::errors::{self, ErrorKind};
use crate::types::{TypeName, Vars};
use crate::{memory::Memory, CPU};

pub fn parse(
    memory: &mut Memory,
    vars: &mut Vars,
    registers: &mut CPU,
    cmd: Vec<&str>,
) -> Result<(), ErrorKind> {
    if cmd.len() < 3 {
        return Err(ErrorKind::ArgErr);
    }

    match cmd[2] {
        ":int" => match cmd[1] {
            "string" => registers.lx = errors::parse_int(&registers.string)?,
            "lxstring" => registers.lx = errors::parse_int(&registers.lxstring)?,
            _ => {
                let t = vars.get_type(cmd[1].to_string())?;
                registers.lx = errors::parse_int(&memory.yeild_string(t.location))?
            }
        },
        ":str" => match cmd[1] {
            "lx" => registers.string = registers.lx.to_string(),
            "rv" => registers.string = registers.rv.to_string(),
            _ => {
                let t = vars.get_type(cmd[1].to_string())?;
                if t.name == TypeName::I32 {
                    registers.string = memory.yeild_i32(t.location).to_string();
                } else {
                    return Err(ErrorKind::ExpectedInt(cmd[1].to_string()));
                }
            }
        },
        _ => return Err(ErrorKind::ArgErr),
    }
    Ok(())
}
