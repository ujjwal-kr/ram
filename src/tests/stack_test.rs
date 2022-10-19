use crate::funcs::stack;
use crate::{memory::Memory, types::Vars, Registers};

#[test]
fn ram_10() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    let statement = "ram 10";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::ram(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );
    assert_eq!(memory.get_int_from_stack(), 10)
}

#[test]
fn ram_lx_10() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    let statement = "ram lx 10";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::ram(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(registers.lx, 10)
}

#[test]
fn ram_rv_10() {
    let mut memory: Memory = Memory::new();
    let mut registers: Registers = Registers::new();
    let mut vars: Vars = Vars::new();

    let statement = "ram rv 10";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::ram(
        &mut memory,
        &mut vars,
        &mut registers,
        cmd,
        statement,
        "main",
        1,
    );

    assert_eq!(registers.rv, 10)
}
