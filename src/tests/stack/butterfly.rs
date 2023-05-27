use crate::funcs::butterfly;
use crate::memory::Location;
use crate::types::{ButterFly, Type};
use crate::{memory::Memory, types::Vars};

#[test]
fn ram_map() {
    let mut memory: Memory = Memory::new();
    let mut vars: Vars = Vars::new();

    let statement: &str = "ram x :map";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    butterfly::map(&mut memory, &mut vars, cmd, statement).unwrap();
    let t: Type = Type {
        name: crate::types::TypeName::ButterFly(ButterFly::new()),
        location: Location { start: 0, size: 0 },
    };
    assert_eq!(vars.get_type("x".to_string()).unwrap(), t)
}

#[test]
fn insert_and_get_map() {
    let mut memory: Memory = Memory::new();
    let mut vars: Vars = Vars::new();

    let statement: &str = "ram x :map";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    butterfly::map(&mut memory, &mut vars, cmd, statement).unwrap();
    let statement: &str = "insert x { \"hello\": \"world\" } ";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    butterfly::map(&mut memory, &mut vars, cmd, statement).unwrap();
    
    vars.set_string("y".to_string(), "", &mut memory);
    let statement: &str = "get x \"hello\" = y";
    let cmd: Vec<&str> = statement.split_whitespace().collect();
    butterfly::map(&mut memory, &mut vars, cmd, statement).unwrap();
    let t = vars.get_type("y".to_string()).unwrap();
    assert_eq!(memory.yeild_string(t.location), "world".to_string());
}