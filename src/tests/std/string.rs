use crate::funcs::stdf::string::*;
use crate::{memory::Memory, types::Vars, Registers};

// split string > "" = var
// split lxstring > " " = var
// split var > "\n" = var

// concat string lxstring
// concat lxstring string
// concat var string
// concat string var
// concat lxstring var
// concat var var

#[test]
pub fn split_string() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    registers.string = "he,llo".to_string();
    vars.set_str_vec("x".to_string(), "[]", &mut memory);
    let statement = "split string > \",\" = x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    split(&mut memory, &mut vars, &mut registers, cmd, statement, "main", 1);
    let t = vars.get_type("x".to_string(), "main", 1);
    assert_eq!(memory.yeild_str_vec(t.location), ["he", "llo"]);
}

#[test]
pub fn split_lxstring() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    registers.lxstring = "hello world".to_string();
    vars.set_str_vec("x".to_string(), "[]", &mut memory);
    let statement = "split lxstring > \" \" = x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    split(&mut memory, &mut vars, &mut registers, cmd, statement, "main", 1);
    let t = vars.get_type("x".to_string(), "main", 1);
    assert_eq!(memory.yeild_str_vec(t.location), ["hello", "world"]);
}

#[test]
pub fn split_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    let statement = "split str > \"\n\" = x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    vars.set_string("str".to_string(), "hello\nworld", &mut memory);
    vars.set_str_vec("x".to_string(), "[]", &mut memory);

    split(&mut memory, &mut vars, &mut registers, cmd, statement, "main", 1);
    let t = vars.get_type("x".to_string(), "main", 1);
    assert_eq!(memory.yeild_str_vec(t.location), ["hello", "world"]);
}