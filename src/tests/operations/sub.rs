use crate::funcs::operations;
use crate::{memory::Memory, types::Vars, CPU};

// sub
// sub lx
// sub rv
// sub var
// sub lx rv
// sub rv lx
// sub var1 var2
// sub var lx
// sub var rv
// sub lx var
// sub rv var

#[test]
fn sub() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    memory.set_int_to_stack(5);
    memory.set_int_to_stack(1);

    let statement = "sub";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::sub::sub(&mut memory, &mut vars, &mut registers, cmd).unwrap();

    assert_eq!(memory.get_int_from_stack().unwrap(), 4)
}

#[test]
fn sub_lx() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    memory.set_int_to_stack(1);
    registers.lx = 2;

    let statement = "sub lx";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::sub::sub(&mut memory, &mut vars, &mut registers, cmd).unwrap();

    assert_eq!(memory.get_int_from_stack().unwrap(), -1)
}

#[test]
fn sub_rv() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    memory.set_int_to_stack(1);
    registers.rv = 2;

    let statement = "sub rv";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::sub::sub(&mut memory, &mut vars, &mut registers, cmd).unwrap();

    assert_eq!(memory.get_int_from_stack().unwrap(), -1)
}

#[test]
fn sub_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_int("x".to_string(), "2", &mut memory).unwrap();
    memory.set_int_to_stack(1);

    let statement = "sub x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::sub::sub(&mut memory, &mut vars, &mut registers, cmd).unwrap();

    assert_eq!(memory.get_int_from_stack().unwrap(), 1)
}

#[test]
fn sub_lx_rv() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    registers.lx = 1;
    registers.rv = 2;

    let statement = "sub lx rv";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::sub::sub(&mut memory, &mut vars, &mut registers, cmd).unwrap();

    assert_eq!(memory.get_int_from_stack().unwrap(), -1)
}

#[test]
fn sub_rv_lx() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    registers.lx = 1;
    registers.rv = 2;

    let statement = "sub rv lx";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::sub::sub(&mut memory, &mut vars, &mut registers, cmd).unwrap();

    assert_eq!(memory.get_int_from_stack().unwrap(), 1)
}

#[test]
fn sub_var_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_int("x".to_string(), "1", &mut memory).unwrap();
    vars.set_int("y".to_string(), "2", &mut memory).unwrap();

    let statement = "sub x y";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::sub::sub(&mut memory, &mut vars, &mut registers, cmd).unwrap();

    assert_eq!(memory.get_int_from_stack().unwrap(), -1)
}

#[test]
fn sub_var_lx() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_int("x".to_string(), "1", &mut memory).unwrap();
    registers.lx = 2;

    let statement = "sub x lx";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::sub::sub(&mut memory, &mut vars, &mut registers, cmd).unwrap();

    assert_eq!(memory.get_int_from_stack().unwrap(), -1)
}

#[test]
fn sub_var_rv() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_int("x".to_string(), "1", &mut memory).unwrap();
    registers.rv = 2;

    let statement = "sub x rv";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::sub::sub(&mut memory, &mut vars, &mut registers, cmd).unwrap();

    assert_eq!(memory.get_int_from_stack().unwrap(), -1)
}

#[test]
fn sub_lx_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_int("x".to_string(), "1", &mut memory).unwrap();
    registers.lx = 2;

    let statement = "sub lx x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::sub::sub(&mut memory, &mut vars, &mut registers, cmd).unwrap();

    assert_eq!(memory.get_int_from_stack().unwrap(), 1)
}

#[test]
fn sub_rv_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_int("x".to_string(), "1", &mut memory).unwrap();
    registers.rv = 2;

    let statement = "sub rv x";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::sub::sub(&mut memory, &mut vars, &mut registers, cmd).unwrap();

    assert_eq!(memory.get_int_from_stack().unwrap(), 1)
}

#[test]
fn sub_lx_lx() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    registers.lx = 3;

    let statement = "sub lx lx";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::sub::sub(&mut memory, &mut vars, &mut registers, cmd).unwrap();

    assert_eq!(memory.get_int_from_stack().unwrap(), 0)
}

#[test]
fn sub_rv_rv() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    registers.rv = 3;

    let statement = "add rv rv";
    let cmd: Vec<&str> = statement.split_whitespace().collect();

    operations::sub::sub(&mut memory, &mut vars, &mut registers, cmd).unwrap();

    assert_eq!(memory.get_int_from_stack().unwrap(), 0)
}
