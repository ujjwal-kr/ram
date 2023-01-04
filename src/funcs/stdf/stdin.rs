use crate::funcs::errors::ErrorKind;
use crate::types::{TypeName, Vars};
use crate::{memory::Memory, CPU};
use std::io::{self, Write};

pub fn stdin(
    memory: &mut Memory,
    vars: &mut Vars,
    registers: &mut CPU,
    cmd: Vec<&str>,
    statement: &str,
) -> Result<(), ErrorKind> {
    // stdin string = "input something: "
    // stdin lxstring = "kekw "
    // stdin var

    if cmd.len() < 2 {
        return Err(ErrorKind::ArgErr);
    }
    let mut message: &str = "";
    if statement.contains('=') {
        let exp = statement.split('=').collect::<Vec<&str>>()[1].trim();
        message = &exp[1..exp.len() - 1];
    }
    print!("{}", message);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Error reading input");
    match cmd[1] {
        "string" => registers.string = buffer,
        "lxstring" => registers.lxstring = buffer,
        _ => {
            let t = vars.get_type(cmd[1].to_string())?;
            if t.name == TypeName::String {
                let heap_addr = u32::from_be_bytes(memory.load(t.location).try_into().unwrap());
                memory.heap_mod(heap_addr, buffer.as_bytes());
            } else {
                return Err(ErrorKind::ExpectedStr(cmd[2].to_string()));
            }
        }
    }

    Ok(())
}
