use crate::funcs::errors;
use crate::types::{TypeName, Vars};
use crate::{memory::Memory, CPU};

pub fn parse(
    memory: &mut Memory,
    vars: &mut Vars,
    registers: &mut CPU,
    cmd: Vec<&str>,
    b: &str,
    l: i32,
) {
    if cmd.len() < 3 {
        errors::args_error(b, l);
    }

    match cmd[2] {
        ":int" => match cmd[1] {
            "string" => {
                registers.lx = registers
                    .string
                    .parse()
                    .expect(format!("Cannot convert to int {}{}", b, l).trim())
            }
            "lxstring" => {
                registers.lx = registers
                    .lxstring
                    .parse()
                    .expect(format!("Cannot convert to int {}{}", b, l).trim())
            }
            _ => {
                let t = vars.get_type(cmd[1].to_string(), b, l);
                if t.name == TypeName::String {
                    registers.lx = memory
                        .yeild_string(t.location)
                        .parse()
                        .expect(format!("Cannot convert to int {}{}", b, l).trim())
                } else {
                    panic!("Cannot convert to int {}{}", b, l)
                }
            }
        },
        ":str" => match cmd[1] {
            "lx" => registers.string = registers.lx.to_string(),
            "rv" => registers.string = registers.rv.to_string(),
            _ => {
                let t = vars.get_type(cmd[1].to_string(), b, l);
                if t.name == TypeName::I32 {
                    registers.string = memory.yeild_i32(t.location).to_string();
                } else {
                    panic!("Cannot convert to int {}{}", b, l)
                }
            }
        },
        _ => errors::args_error(b, l),
    }
}
