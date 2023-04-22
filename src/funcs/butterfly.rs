use super::errors::ErrorKind;
use crate::{memory::Memory, types::Vars, CPU};

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
        "ram" => {
            vars.add_map(cmd[1].to_string());
        }
        "get" => todo!(),
        "insert" => todo!(),
        "delete" => todo!(),
        _ => return Err(ErrorKind::ArgErr),
    }
    Ok(())
}
