use super::super::funcs::operations;
use super::*;

pub fn add_works() {
    // add
    let mut statement: &str = "add";
    let mut vars = super::super::Vars {
        lx: 0.0,
        rv: 0.0,
        string: "".to_string(),
        lxstring: "".to_string(),
        num_vec: vec![],
        str_vec: vec![],
    };
    let mut stack: Vec<f64> = vec![];
    stack.push(2.0);
    stack.push(3.0);
    let mut cmd: Vec<&str> = vec![];
    cmd.push("add");
    operations::add(&mut stack, cmd, &mut vars, 0, 1);
    assert_f64(stack[2], 5.0, statement);

    // add lx rv
    stack = vec![];
    statement = "add lx rv";
    vars.lx = 2.0;
    vars.rv = 1.0;
    cmd = statement.split(" ").collect();
    operations::add(&mut stack, cmd, &mut vars, 0, 1);
    assert_f64(stack[0], 3.0, statement);
}
