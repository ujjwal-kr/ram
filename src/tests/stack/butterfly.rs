use crate::funcs::butterfly;
use crate::memory::Location;
use crate::types::{ButterFly, Type};
use crate::{memory::Memory, types::Vars, CPU};

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
