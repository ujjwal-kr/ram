use super::super::funcs::operations;
use super::*;
use std::collections::HashMap;

pub fn operation_works() {
    add_works();
    sub_works();
    mul_works();
    div_works();
    sqr_works();
    sqrt_works();
    round_works();
    avg_works();
    split_works();
    vec_ops_works();
}

fn add_works() {
    // add
    let mut statement: &str = "add";
    let mut vars = super::super::Vars {
        lx: 0.0,
        rv: 0.0,
        string: "".to_string(),
        lxstring: "".to_string(),
        num_vec: vec![],
        str_vec: vec![],
        var_str: HashMap::new(),
        var_int: HashMap::new(),
        var_str_vec: HashMap::new(),
        var_int_vec: HashMap::new()
    };
    let mut stack: Vec<f64> = vec![];
    stack.push(2.0);
    stack.push(3.0);
    let mut cmd: Vec<&str> = vec![];
    cmd.push("add");
    operations::add(&mut stack, cmd, &mut vars, "0", 1);
    assert_f64(stack[2], 5.0, statement);

    // add lx rv
    stack = vec![];
    statement = "add lx rv";
    vars.lx = 2.0;
    vars.rv = 1.0;
    cmd = statement.split(" ").collect();
    operations::add(&mut stack, cmd, &mut vars, "0", 1);
    assert_f64(stack[0], 3.0, statement);
}

fn sub_works() {
    // sub
    let statement: &str = "sub";
    let mut stack: Vec<f64> = vec![];
    stack.push(3.0);
    stack.push(1.0);
    operations::sub(&mut stack, "0", 1);
    assert_f64(stack[2], 2.0, statement);
}

fn mul_works() {
    // mul
    let mut statement: &str = "mul";
    let mut vars = super::super::Vars {
        lx: 0.0,
        rv: 0.0,
        string: "".to_string(),
        lxstring: "".to_string(),
        num_vec: vec![],
        str_vec: vec![],
        var_str: HashMap::new(),
        var_int: HashMap::new(),
        var_str_vec: HashMap::new(),
        var_int_vec: HashMap::new()
    };
    let mut stack: Vec<f64> = vec![];
    stack.push(2.0);
    stack.push(3.0);
    let mut cmd: Vec<&str> = vec![];
    cmd.push("mul");
    operations::mul(&mut stack, cmd, &mut vars, "0", 1);
    assert_f64(stack[2], 6.0, statement);

    // mul lx rv
    stack = vec![];
    statement = "mul lx rv";
    vars.lx = 2.0;
    vars.rv = 1.0;
    cmd = statement.split(" ").collect();
    operations::mul(&mut stack, cmd, &mut vars, "0", 1);
    assert_f64(stack[0], 2.0, statement);
}

fn div_works() {
    // div
    let statement: &str = "div";
    let mut stack: Vec<f64> = vec![];
    stack.push(8.0);
    stack.push(2.0);
    operations::div(&mut stack, "0", 1);
    assert_f64(stack[2], 4.0, statement);
}

fn sqr_works() {
    // sqr
    let mut statement: &str = "sqr";
    let mut vars = super::super::Vars {
        lx: 0.0,
        rv: 0.0,
        string: "".to_string(),
        lxstring: "".to_string(),
        num_vec: vec![],
        str_vec: vec![],
        var_str: HashMap::new(),
        var_int: HashMap::new(),
        var_str_vec: HashMap::new(),
        var_int_vec: HashMap::new()
    };

    let mut stack: Vec<f64> = vec![];
    stack.push(5.0);
    let mut cmd: Vec<&str> = vec![];
    cmd.push("sqr");
    operations::sqr(&mut stack, cmd, &mut vars, "0", 1);
    assert_f64(stack[1], 25.0, statement);

    // sqr lx
    stack = vec![];
    statement = "sqr lx";
    vars.lx = 3.0;
    cmd = statement.split(" ").collect();
    operations::sqr(&mut stack, cmd, &mut vars, "0", 1);
    assert_f64(vars.lx, 9.0, statement);

    // sqr rv
    stack = vec![];
    statement = "sqr rv";
    vars.rv = 4.0;
    cmd = statement.split(" ").collect();
    operations::sqr(&mut stack, cmd, &mut vars, "0", 1);
    assert_f64(vars.rv, 16.0, statement);
}

fn sqrt_works() {
    // sqrt
    let mut statement: &str = "sqrt";
    let mut vars = super::super::Vars {
        lx: 0.0,
        rv: 0.0,
        string: "".to_string(),
        lxstring: "".to_string(),
        num_vec: vec![],
        str_vec: vec![],
        var_str: HashMap::new(),
        var_int: HashMap::new(),
        var_str_vec: HashMap::new(),
        var_int_vec: HashMap::new()
    };

    let mut stack: Vec<f64> = vec![];
    stack.push(9.0);
    let mut cmd: Vec<&str> = vec![];
    cmd.push("sqrt");
    operations::sqrt(&mut stack, cmd, &mut vars, "0", 1);
    assert_f64(stack[1], 3.0, statement);

    // sqr lx
    stack = vec![];
    statement = "sqrt lx";
    vars.lx = 25.0;
    cmd = statement.split(" ").collect();
    operations::sqrt(&mut stack, cmd, &mut vars, "0", 1);
    assert_f64(vars.lx, 5.0, statement);

    // sqr rv
    stack = vec![];
    statement = "sqrt rv";
    vars.rv = 4.0;
    cmd = statement.split(" ").collect();
    operations::sqrt(&mut stack, cmd, &mut vars, "0", 1);
    assert_f64(vars.rv, 2.0, statement);
}

fn round_works() {
    // round
    let mut statement: &str = "round";
    let mut vars = super::super::Vars {
        lx: 0.0,
        rv: 0.0,
        string: "".to_string(),
        lxstring: "".to_string(),
        num_vec: vec![],
        str_vec: vec![],
        var_str: HashMap::new(),
        var_int: HashMap::new(),
        var_str_vec: HashMap::new(),
        var_int_vec: HashMap::new()
    };

    let mut stack: Vec<f64> = vec![];
    stack.push(9.3);
    let mut cmd: Vec<&str> = vec![];
    cmd.push("round");
    operations::round(&mut stack, cmd, &mut vars, "0", 1);
    assert_f64(stack[1], 9.0, statement);

    // round lx
    stack = vec![];
    statement = "round lx";
    vars.lx = 25.7;
    cmd = statement.split(" ").collect();
    operations::round(&mut stack, cmd, &mut vars, "0", 1);
    assert_f64(vars.lx, 26.0, statement);

    // round rv
    stack = vec![];
    statement = "round rv";
    vars.rv = 4.5;
    cmd = statement.split(" ").collect();
    operations::round(&mut stack, cmd, &mut vars, "0", 1);
    assert_f64(vars.rv, 5.0, statement);
}

fn avg_works() {
    // avg
    let statement: &str = "avg";
    let mut stack: Vec<f64> = vec![];
    stack.push(10.0);
    stack.push(12.0);
    stack.push(2.0);
    operations::avg(&mut stack, "0", 1);
    assert_f64(stack[3], 8.0, statement);
}

fn split_works() {
    // split >> ","
    let mut vars = super::super::Vars {
        lx: 0.0,
        rv: 0.0,
        string: "".to_string(),
        lxstring: "".to_string(),
        num_vec: vec![],
        str_vec: vec![],
        var_str: HashMap::new(),
        var_int: HashMap::new(),
        var_str_vec: HashMap::new(),
        var_int_vec: HashMap::new()
    };
    vars.string = "one,two,three".to_string();
    let statement: &str = "split >> \",\"";
    let cmd = statement.split(" ").collect();
    operations::split(cmd, statement, &mut vars, "0", 1);
    assert_vec_str(vars.str_vec, ["one", "two", "three"].to_vec(), statement);
}

fn vec_ops_works() {
    // vec str push
    let mut stack: Vec<f64> = vec![];
    let mut statement: &str = "vec str push";
    let mut cmd: Vec<&str> = statement.split(" ").collect();
    let mut vars = super::super::Vars {
        lx: 0.0,
        rv: 0.0,
        string: "".to_string(),
        lxstring: "".to_string(),
        num_vec: vec![],
        str_vec: vec![],
        var_str: HashMap::new(),
        var_int: HashMap::new(),
        var_str_vec: HashMap::new(),
        var_int_vec: HashMap::new()
    };

    vars.str_vec.push("hello".to_string()); // at place 0
    vars.string = "hello2".to_string();
    operations::vec_ops(&mut stack, cmd, statement, &mut vars, "0", 1); // push string at [1]
    assert_vec_str(
        vars.str_vec.clone(),
        ["hello", "hello2"].to_vec(),
        statement,
    );
    vars.str_vec = vec![];

    // vec int push lx
    statement = "vec int push lx";
    vars.lx = 5.0;
    vars.num_vec.push(3.0);
    cmd = statement.split(" ").collect();
    operations::vec_ops(&mut stack, cmd, statement, &mut vars, "0", 1);
    assert_vec_int(vars.num_vec.clone(), [3.0, 5.0].to_vec(), statement);
    vars.num_vec = vec![];

    // vec int push rv
    statement = "vec int push rv";
    vars.rv = 4.0;
    vars.num_vec.push(3.0);
    cmd = statement.split(" ").collect();
    operations::vec_ops(&mut stack, cmd, statement, &mut vars, "0", 1);
    assert_vec_int(vars.num_vec.clone(), [3.0, 4.0].to_vec(), statement);
    vars.num_vec = vec![];

    // vec int lx >> [1]
    statement = "vec int lx >> [1]";
    vars.num_vec.push(1.0);
    vars.num_vec.push(3.0);
    cmd = statement.split(" ").collect();
    operations::vec_ops(&mut stack, cmd, statement, &mut vars, "0", 1);
    assert_f64(vars.lx, vars.num_vec[1], statement);
    vars.num_vec = vec![];

    // vec int rv >> [0]
    statement = "vec int rv >> [0]";
    vars.num_vec.push(1.0);
    vars.num_vec.push(3.0);
    cmd = statement.split(" ").collect();
    operations::vec_ops(&mut stack, cmd, statement, &mut vars, "0", 1);
    assert_f64(vars.rv, vars.num_vec[0], statement);
    vars.num_vec = vec![];

    // vec int rv >> [lx]
    statement = "vec int rv >> [lx]";
    vars.num_vec.push(1.0);
    vars.num_vec.push(3.0);
    vars.lx = 1.0;
    cmd = statement.split(" ").collect();
    operations::vec_ops(&mut stack, cmd, statement, &mut vars, "0", 1);
    assert_f64(vars.rv, vars.num_vec[1], statement);
    vars.num_vec = vec![];

    // vec int lx >> [rv]
    statement = "vec int lx >> [rv]";
    vars.num_vec.push(1.0);
    vars.num_vec.push(3.0);
    vars.rv = 1.0;
    cmd = statement.split(" ").collect();
    operations::vec_ops(&mut stack, cmd, statement, &mut vars, "0", 1);
    assert_f64(vars.lx, vars.num_vec[1], statement);
    vars.num_vec = vec![];

    // vec str >> [1]
    statement = "vec str >> [1]";
    cmd = statement.split(" ").collect();
    vars.str_vec.push("one".to_string());
    vars.str_vec.push("two".to_string());
    operations::vec_ops(&mut stack, cmd, statement, &mut vars, "0", 1);
    assert_str(vars.string.trim(), vars.str_vec[1].trim(), statement);
    vars.str_vec = vec![];

    // vec str >> [lx]
    statement = "vec str >> [lx]";
    vars.lx = 1.0;
    cmd = statement.split(" ").collect();
    vars.str_vec.push("one".to_string());
    vars.str_vec.push("two".to_string());
    operations::vec_ops(&mut stack, cmd, statement, &mut vars, "0", 1);
    assert_str(vars.string.trim(), vars.str_vec[1].trim(), statement);
    vars.str_vec = vec![];

    // vec str >> [rv]
    statement = "vec str >> [rv]";
    vars.rv = 1.0;
    cmd = statement.split(" ").collect();
    vars.str_vec.push("one".to_string());
    vars.str_vec.push("two".to_string());
    operations::vec_ops(&mut stack, cmd, statement, &mut vars, "0", 1);
    assert_str(vars.string.trim(), vars.str_vec[1].trim(), statement);
    vars.str_vec = vec![];

    // vec str len
    statement = "vec str len";
    vars.str_vec.push("one".to_string());
    vars.str_vec.push("two".to_string());
    cmd = statement.split(" ").collect();
    operations::vec_ops(&mut stack, cmd, statement, &mut vars, "0", 1);
    assert_f64(stack[0], 2.0, statement);
    stack = vec![];

    // vec int len
    statement = "vec int len";
    cmd = statement.split(" ").collect();
    vars.num_vec.push(2.0);
    operations::vec_ops(&mut stack, cmd, statement, &mut vars, "0", 1);
    assert_f64(stack[0], 1.0, statement);

    // vec str pop
    vars.str_vec = vec!["test".to_string(), "test1".to_string()];
    statement = "vec str pop";
    cmd = statement.split(" ").collect();
    operations::vec_ops(&mut stack, cmd, statement, &mut vars, "0", 1);
    assert_vec_string(
        ["test".to_string()].to_vec(),
        vars.str_vec.clone(),
        statement,
    );

    // vec int pop
    vars.num_vec = vec![10.0, 1.0];
    statement = "vec int pop";
    cmd = statement.split(" ").collect();
    operations::vec_ops(&mut stack, cmd, statement, &mut vars, "0", 1);
    assert_vec_int(vars.num_vec, [10.0].to_vec(), statement);
}
