use crate::memory::Memory;
use crate::{Registers, Vars};

pub fn print(
    memory: &mut Memory,
    vars: &mut Vars,
    registers: &mut Registers,
    cmd: Vec<&str>,
    statement: &str,
    b: &str,
    l: i32,
) {
    // print var <var_name> { lx/rv/string/lxstring/var-name }
    // print hello world

    if cmd[1] == "var" {
        match cmd[2] {
            "lx" => println!("{}", registers.lx),
            "rv" => println!("{}", registers.rv),
            "string" => println!("{}", registers.string),
            "lxstring" => println!("{}", registers.lxstring),
            _ => {
                if cmd.len() == 3 {
                } else {
                    let print_st = &cmd[1..cmd.len()].to_vec().join(" ").to_string();
                    println!("{print_st}");
                }
            }
        }
    } else {
        let print_st = &cmd[1..cmd.len()].to_vec().join(" ").to_string();
        println!("{print_st}");
    }
}
