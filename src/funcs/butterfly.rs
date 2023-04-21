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

    todo!()
}
