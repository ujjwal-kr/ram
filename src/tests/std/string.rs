use crate::funcs::stdf::string::*;
use crate::{memory::Memory, types::Vars, CPU};

// split string > "" = var
// split lxstring > " " = var
// split var > "\n" = var

// concat string lxstring
// concat string var

// concat lxstring string
// concat lxstring var

// concat var string
// concat var var

#[test]
pub fn split_string() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    registers.string = "he,llo".to_string();
    vars.set_str_vec("x".to_string(), "[]", &mut memory);
    let statement = "split string > \",\" = x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    split(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );
    let t = vars.get_type("x".to_string(), "main", 1);
    assert_eq!(memory.yeild_str_vec(t.location), ["he", "llo"]);
}

#[test]
pub fn split_lxstring() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    registers.lxstring = "hello world".to_string();
    vars.set_str_vec("x".to_string(), "[]", &mut memory);
    let statement = "split lxstring > \" \" = x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    split(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );
    let t = vars.get_type("x".to_string(), "main", 1);
    assert_eq!(memory.yeild_str_vec(t.location), ["hello", "world"]);
}

#[test]
pub fn split_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    let statement = "split str > \"\n\" = x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    vars.set_string("str".to_string(), "hello\nworld", &mut memory);
    vars.set_str_vec("x".to_string(), "[]", &mut memory);

    split(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );
    let t = vars.get_type("x".to_string(), "main", 1);
    assert_eq!(memory.yeild_str_vec(t.location), ["hello", "world"]);
}

#[test]
pub fn concat_string_lxstring() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    registers.string = "hello ".to_string();
    registers.lxstring = "world".to_string();
    let statement = "concat string lxstring";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    concat(&mut memory, &mut vars, &mut registers, cmd, "main", 1);
    assert_eq!(registers.string, "hello world".to_string())
}

pub fn concat_lxstring_string() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    registers.lxstring = "hello ".to_string();
    registers.string = "world".to_string();
    let statement = "concat lxstring string";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    concat(&mut memory, &mut vars, &mut registers, cmd, "main", 1);
    assert_eq!(registers.string, "hello world".to_string())
}

pub fn concat_var_string() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_string("x".to_string(), "hello ", &mut memory);
    registers.string = "world".to_string();
    let statement = "concat x string";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    concat(&mut memory, &mut vars, &mut registers, cmd, "main", 1);
    assert_eq!(registers.string, "hello world".to_string())
}

#[test]
pub fn concat_string_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_string("x".to_string(), "world", &mut memory);
    registers.string = "hello ".to_string();
    let statement = "concat string x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    concat(&mut memory, &mut vars, &mut registers, cmd, "main", 1);
    assert_eq!(registers.string, "hello world".to_string())
}

#[test]
pub fn concat_lxstring_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_string("x".to_string(), "world", &mut memory);
    registers.lxstring = "hello ".to_string();
    let statement = "concat lxstring x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    concat(&mut memory, &mut vars, &mut registers, cmd, "main", 1);
    assert_eq!(registers.string, "hello world".to_string())
}

#[test]
pub fn concat_var_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_string("x".to_string(), "hello ", &mut memory);
    vars.set_string("y".to_string(), "world", &mut memory);
    let statement = "concat x y";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    concat(&mut memory, &mut vars, &mut registers, cmd, "main", 1);
    assert_eq!(registers.string, "hello world".to_string())
}
