use super::super::funcs::stack;
use super::*;
use std::collections::HashMap;

pub fn ram_works() {
    // ram 10
    let mut stack: Vec<f64> = vec![];
    let mut statement: &str = "ram 10";
    let mut cmd: Vec<&str> = statement.split(" ").collect();
    let mut vars = super::super::Vars {
        lx: 0.0,
        rv: 0.0,
        string: "".to_string(),
        lxstring: "".to_string(),
        num_vec: vec![],
        str_vec: vec![],
        hash_str: HashMap::new(),
        hash_int: HashMap::new(),
        hash_int_vec: HashMap::new(),
        hash_str_vec: HashMap::new(),
    };

    stack::ram(&mut stack, cmd, statement, &mut vars, 0, 1);
    assert_f64(stack[0], 10.0, statement);

    // ram lx 12
    statement = "ram lx 12";
    cmd = statement.split(" ").collect();
    stack = vec![];

    stack::ram(&mut stack, cmd, statement, &mut vars, 0, 1);
    assert_f64(vars.lx, 12.0, statement);

    // ram rv 13
    statement = "ram rv 13";
    cmd = statement.split(" ").collect();
    stack = vec![];
    stack::ram(&mut stack, cmd, statement, &mut vars, 0, 1);
    assert_f64(vars.rv, 13.0, statement);

    // ram lx
    vars.lx = 5.0;
    statement = "ram lx";
    cmd = statement.split(" ").collect();
    stack = vec![];
    stack::ram(&mut stack, cmd, statement, &mut vars, 0, 1);
    assert_f64(vars.lx, stack[0], statement);

    // ram rv
    vars.rv = 5.0;
    statement = "ram rv";
    cmd = statement.split(" ").collect();
    stack = vec![];
    stack::ram(&mut stack, cmd, statement, &mut vars, 0, 1);
    assert_f64(vars.rv, stack[0], statement);

    // ram lx prev
    stack = vec![];
    stack.push(2.0);
    statement = "ram lx prev";
    cmd = statement.split(" ").collect();
    stack::ram(&mut stack, cmd, statement, &mut vars, 0, 1);
    assert_f64(stack[0], vars.lx, statement);

    // ram rv prev
    stack = vec![];
    stack.push(3.0);
    statement = "ram rv prev";
    cmd = statement.split(" ").collect();
    stack::ram(&mut stack, cmd, statement, &mut vars, 0, 1);
    assert_f64(stack[0], vars.rv, statement);

    // ram string >> hello world
    stack = vec![];
    statement = "ram string >> hello world";
    cmd = statement.split(" ").collect();
    stack::ram(&mut stack, cmd, statement, &mut vars, 0, 1);
    assert_str("hello world", vars.string.trim(), statement);

    // ram lxstring >> hello world
    stack = vec![];
    statement = "ram lxstring >> hello world";
    cmd = statement.split(" ").collect();
    stack::ram(&mut stack, cmd, statement, &mut vars, 0, 1);
    assert_str("hello world", vars.lxstring.trim(), statement);

    // ram vec int >> [1, 2, 3]
    statement = "ram vec int >> [1, 2, 3]";
    cmd = statement.split(" ").collect();
    stack::ram(&mut stack, cmd, statement, &mut vars, 0, 1);
    assert_vec_int(vars.num_vec.clone(), [1.0, 2.0, 3.0].to_vec(), statement);

    // ram vec str >> [1, 2, 3]
    statement = "ram vec str >> [hello, a, b]";
    cmd = statement.split(" ").collect();
    stack::ram(&mut stack, cmd, statement, &mut vars, 0, 1);
    assert_vec_str(vars.str_vec, ["hello", "a", "b"].to_vec(), statement);
}

pub fn str_works() {
    // str lxstring string
    let mut statement: &str = "str lxstring string";
    let mut stack: Vec<f64> = vec![];
    let mut cmd: Vec<&str> = statement.split(" ").collect();
    let mut vars = super::super::Vars {
        lx: 0.0,
        rv: 0.0,
        string: "".to_string(),
        lxstring: "".to_string(),
        num_vec: vec![],
        str_vec: vec![],
        hash_str: HashMap::new(),
        hash_int: HashMap::new(),
        hash_int_vec: HashMap::new(),
        hash_str_vec: HashMap::new(),
    };

    vars.string = "helloworld".to_string();
    stack::strfn(&mut stack, &mut vars, cmd, 0, 1);
    assert_str(vars.lxstring.trim(), vars.string.trim(), statement);

    // str string lxstring
    statement = "str string lxstring";
    cmd = statement.split(" ").collect();
    vars.lxstring = "abc".to_string();
    stack::strfn(&mut stack, &mut vars, cmd, 0, 1);
    assert_str(vars.lxstring.trim(), vars.string.trim(), statement);
}
