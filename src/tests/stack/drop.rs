use crate::funcs::stack::drop;
use crate::{memory::Memory, types::Vars};

#[test]
#[should_panic]
fn drop_str() {
    let mut memory: Memory = Memory::new();
    let mut vars: Vars = Vars::new();
    vars.set_string("x".to_string(), "ok", &mut memory); 
    let statement: &str = "drop x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    drop(&mut memory, &mut vars, cmd).unwrap();
    vars.get_type("x".to_string()).unwrap();
}

#[test]
#[should_panic]
fn drop_int_vec() {
    let mut memory: Memory = Memory::new();
    let mut vars: Vars = Vars::new();
    vars.set_int_vec("x".to_string(), "[1, 2, 3]", &mut memory).unwrap(); 
    let statement: &str = "drop x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    drop(&mut memory, &mut vars, cmd).unwrap();
    vars.get_type("x".to_string()).unwrap();
}

#[test]
#[should_panic]
fn drop_str_vec() {
    let mut memory: Memory = Memory::new();
    let mut vars: Vars = Vars::new();
    vars.set_str_vec("x".to_string(), "['a', 'b', 'c']", &mut memory); 
    let statement: &str = "drop x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    drop(&mut memory, &mut vars, cmd).unwrap();
    vars.get_type("x".to_string()).unwrap();
}

