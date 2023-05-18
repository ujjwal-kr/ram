use crate::funcs::stdf::string::*;
use crate::{memory::Memory, types::Vars, CPU};

// split string > "" = var
// split lxstring > " " = var
// split var > "\n" = var

// join var > "" = string
// join var > " " = lxstring
// join var > "\n" = var

// concat string lxstring
// concat string var

// concat lxstring string
// concat lxstring var

// concat var string
// concat var var

// trim string
// trim lxstring
// trim var

#[test]
pub fn trim_string() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    registers.string = " hello".to_string();
    let statement = "trim string";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    trim(&mut memory, &mut vars, &mut registers, cmd);
    assert_eq!(registers.string, "hello".to_string());
}

#[test]
pub fn trim_lxstring() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    registers.lxstring = " hello".to_string();
    let statement = "trim string";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    trim(&mut memory, &mut vars, &mut registers, cmd);
    assert_eq!(registers.lxstring, "hello".to_string());
}

#[test]
pub fn trim_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_string("x".to_string(), " hello", &mut memory);
    let statement = "trim x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    trim(&mut memory, &mut vars, &mut registers, cmd);
    let t = vars.get_type("x".to_string()).unwrap();
    assert_eq!(memory.yeild_string(t.location), "hello".to_string());
}



#[test]
pub fn split_string() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    registers.string = "he,llo".to_string();
    vars.set_str_vec("x".to_string(), "['']", &mut memory);
    let statement = "split string > \",\" = x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    split(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    let t = vars.get_type("x".to_string()).unwrap();
    assert_eq!(memory.yeild_str_vec(t.location), ["he", "llo"]);
}

#[test]
pub fn split_lxstring() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    registers.lxstring = "hello world".to_string();
    vars.set_str_vec("x".to_string(), "['']", &mut memory);
    let statement = "split lxstring > \" \" = x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    split(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    let t = vars.get_type("x".to_string()).unwrap();
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
    vars.set_str_vec("x".to_string(), "['']", &mut memory);

    split(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    let t = vars.get_type("x".to_string()).unwrap();
    assert_eq!(memory.yeild_str_vec(t.location), ["hello", "world"]);
}

// join var > "" = string
// join var > " " = lxstring
// join var > "\n" = var

#[test]
fn join_string() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_str_vec("x".to_string(), "['hello', 'world']", &mut memory);
    let statement = "join x > \"\" = string";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    join(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    assert_eq!(registers.string, "helloworld".to_string());
}

#[test]
fn join_lxstring() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_str_vec("x".to_string(), "['hello', 'world']", &mut memory);
    let statement = "join x > \" \" = lxstring";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    join(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    assert_eq!(registers.string, "hello world".to_string());
}

#[test]
fn join_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_str_vec("x".to_string(), "['hello', 'world']", &mut memory);
    vars.set_string("y".to_string(), "''", &mut memory);
    let statement = "join x > \"\n\" = y";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    join(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    let t = vars.get_type("y".to_string()).unwrap();
    assert_eq!(memory.yeild_string(t.location), "hello\nworld".to_string());
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

    concat(&mut memory, &mut vars, &mut registers, cmd).unwrap();
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

    concat(&mut memory, &mut vars, &mut registers, cmd).unwrap();
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

    concat(&mut memory, &mut vars, &mut registers, cmd).unwrap();
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

    concat(&mut memory, &mut vars, &mut registers, cmd).unwrap();
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

    concat(&mut memory, &mut vars, &mut registers, cmd).unwrap();
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

    concat(&mut memory, &mut vars, &mut registers, cmd).unwrap();
    assert_eq!(registers.string, "hello world".to_string())
}
