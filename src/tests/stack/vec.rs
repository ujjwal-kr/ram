use crate::funcs::stack;
use crate::{memory::Memory, types::Vars, CPU};

// vec x push y
// vec x len
// vec y = x[1]
// vec x[1] = y

#[test]
fn vec_int_push_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_int_vec("x".to_string(), "[1, 2]", &mut memory)
        .unwrap();
    vars.set_int("y".to_string(), "1", &mut memory).unwrap();
    let statement = "vec x push y";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();

    let t = vars.get_type("x".to_string()).unwrap();
    assert_eq!(memory.yeild_int_vec(t.location), [1, 2, 1].to_vec())
}

#[test]
fn vec_int_push_lx() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_int_vec("x".to_string(), "[1, 2]", &mut memory)
        .unwrap();
    registers.lx = 1;
    let statement = "vec x push lx";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();

    let t = vars.get_type("x".to_string()).unwrap();
    assert_eq!(memory.yeild_int_vec(t.location), [1, 2, 1].to_vec())
}

#[test]
fn vec_int_push_rv() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_int_vec("x".to_string(), "[1, 2]", &mut memory)
        .unwrap();
    registers.rv = 1;
    let statement = "vec x push rv";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();

    let t = vars.get_type("x".to_string()).unwrap();
    assert_eq!(memory.yeild_int_vec(t.location), [1, 2, 1].to_vec())
}

// str push

#[test]
fn vec_str_push_string() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_str_vec("x".to_string(), "['a']", &mut memory);
    registers.string = "k".to_string();
    let statement = "vec x push string";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();

    let t = vars.get_type("x".to_string()).unwrap();
    assert_eq!(
        memory.yeild_str_vec(t.location),
        ["a".to_string(), "k".to_string()].to_vec()
    )
}

#[test]
fn vec_str_push_lxstring() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_str_vec("x".to_string(), "['a']", &mut memory);
    registers.lxstring = "k".to_string();
    let statement = "vec x push lxstring";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();

    let t = vars.get_type("x".to_string()).unwrap();
    assert_eq!(
        memory.yeild_str_vec(t.location),
        ["a".to_string(), "k".to_string()].to_vec()
    )
}

#[test]
fn vec_str_push_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_str_vec("x".to_string(), "['a']", &mut memory);
    vars.set_string("y".to_string(), "k", &mut memory);
    let statement = "vec x push y";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();

    let t = vars.get_type("x".to_string()).unwrap();
    assert_eq!(
        memory.yeild_str_vec(t.location),
        ["a".to_string(), "k".to_string()].to_vec()
    )
}

// return by index

// vec lx = x[1]
// vec rv = x[1]
// vec lx = x[rv]

// vec string = x[1]
// vec lxstring = x[1]
// vec string = x[lx]

// vec y = x[1]
// vec lx = x[y]

#[test]
fn vec_get_lx_idx() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_int_vec("x".to_string(), "[1, 2]", &mut memory)
        .unwrap();
    let statement = "vec lx = x[1]";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    assert_eq!(registers.lx, 2);
}

#[test]
fn vec_get_rv_idx() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_int_vec("x".to_string(), "[1, 2]", &mut memory)
        .unwrap();
    let statement = "vec rv = x[1]";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    assert_eq!(registers.rv, 2);
}

#[test]
fn vec_get_lx_rv_idx() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    registers.rv = 1;
    vars.set_int_vec("x".to_string(), "[1, 2]", &mut memory)
        .unwrap();
    let statement = "vec lx = x[rv]";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    assert_eq!(registers.lx, 2);
}

#[test]
fn vec_get_string_idx() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_str_vec("x".to_string(), "['ok', 'omk']", &mut memory);
    let statement = "vec string = x[1]";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    assert_eq!(registers.string, "omk".to_string());
}

#[test]
fn vec_get_lxstring_idx() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();

    vars.set_str_vec("x".to_string(), "['ok', 'omk']", &mut memory);
    let statement = "vec lxstring = x[1]";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    assert_eq!(registers.lxstring, "omk".to_string());
}

#[test]
fn vec_get_string_idx_rv() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();
    registers.rv = 1;
    vars.set_str_vec("x".to_string(), "['ok', 'omk']", &mut memory);
    let statement = "vec string = x[rv]";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    assert_eq!(registers.string, "omk".to_string());
}

#[test]
fn vec_get_var_idx() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();
    vars.set_string("y".to_string(), "''", &mut memory);
    vars.set_str_vec("x".to_string(), "['ok', 'omk']", &mut memory);
    let statement = "vec y = x[1]";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    let t = vars.get_type("y".to_string()).unwrap();
    assert_eq!(memory.yeild_string(t.location), "omk".to_string());
}

#[test]
fn vec_get_var_idx_lx() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();
    vars.set_int("y".to_string(), "1", &mut memory).unwrap();
    vars.set_int_vec("x".to_string(), "[2,4]", &mut memory)
        .unwrap();
    let statement = "vec lx = x[y]";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    let t = vars.get_type("y".to_string()).unwrap();
    assert_eq!(registers.lx, 4);
}

// vec mods

// vec x[1] = lx
// vec x[lx] = rv

// vec x[1] = string
// vec x[lx] = lxstring

// vec x[y] = string
// vec x[1] = y

#[test]
fn vec_mod_lx() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();
    registers.lx = 5;
    vars.set_int_vec("x".to_string(), "[2,4]", &mut memory)
        .unwrap();
    let statement = "vec x[1] = lx";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    let t = vars.get_type("x".to_string()).unwrap();
    assert_eq!(memory.yeild_int_vec(t.location)[1], 5);
}

#[test]
fn vec_mod_lx_rv() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();
    registers.lx = 1;
    registers.rv = 5;
    vars.set_int_vec("x".to_string(), "[2,4]", &mut memory)
        .unwrap();
    let statement = "vec x[lx] = rv";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    let t = vars.get_type("x".to_string()).unwrap();
    assert_eq!(memory.yeild_int_vec(t.location)[1], 5);
}

#[test]
fn vec_mod_string() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();
    registers.string = "omk".to_string();
    vars.set_str_vec("x".to_string(), "['a', 'k']", &mut memory);
    let statement = "vec x[1] = string";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    let t = vars.get_type("x".to_string()).unwrap();
    assert_eq!(memory.yeild_str_vec(t.location)[1], "omk".to_string());
}

#[test]
fn vec_mod_lx_string() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();
    registers.lxstring = "omk".to_string();
    registers.lx = 1;
    vars.set_str_vec("x".to_string(), "['a', 'k']", &mut memory);
    let statement = "vec x[lx] = lxstring";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    let t = vars.get_type("x".to_string()).unwrap();
    assert_eq!(memory.yeild_str_vec(t.location)[1], "omk".to_string());
}

#[test]
fn vec_mod_var_string() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();
    registers.string = "omk".to_string();
    vars.set_int("y".to_string(), "1", &mut memory).unwrap();
    vars.set_str_vec("x".to_string(), "['a', 'k']", &mut memory);
    let statement = "vec x[y] = string";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    let t = vars.get_type("x".to_string()).unwrap();
    assert_eq!(memory.yeild_str_vec(t.location)[1], "omk".to_string());
}

#[test]
fn vec_mod_var() {
    let mut memory: Memory = Memory::new();
    let mut registers: CPU = CPU::new();
    let mut vars: Vars = Vars::new();
    vars.set_int("y".to_string(), "5", &mut memory).unwrap();
    vars.set_int_vec("x".to_string(), "[0, 1]", &mut memory)
        .unwrap();
    let statement = "vec x[i] = y";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    stack::vec(&mut memory, &mut vars, &mut registers, cmd, statement).unwrap();
    let t = vars.get_type("x".to_string()).unwrap();
    assert_eq!(memory.yeild_int_vec(t.location)[1], 5);
}
