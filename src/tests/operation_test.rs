use super::super::funcs::operations;
use super::*;

pub fn operation_works() {
    add_works();
    sub_works();
    mul_works();
    div_works();
    sqr_works();
    sqrt_works();
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

fn sub_works() {
    // sub
    let statement: &str = "sub";
    let mut stack: Vec<f64> = vec![];
    stack.push(3.0);
    stack.push(1.0);
    operations::sub(&mut stack, 0, 1);
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
    };
    let mut stack: Vec<f64> = vec![];
    stack.push(2.0);
    stack.push(3.0);
    let mut cmd: Vec<&str> = vec![];
    cmd.push("mul");
    operations::mul(&mut stack, cmd, &mut vars, 0, 1);
    assert_f64(stack[2], 6.0, statement);

    // mul lx rv
    stack = vec![];
    statement = "mul lx rv";
    vars.lx = 2.0;
    vars.rv = 1.0;
    cmd = statement.split(" ").collect();
    operations::mul(&mut stack, cmd, &mut vars, 0, 1);
    assert_f64(stack[0], 2.0, statement);
}

fn div_works() {
    // div
    let statement: &str = "div";
    let mut stack: Vec<f64> = vec![];
    stack.push(8.0);
    stack.push(2.0);
    operations::div(&mut stack, 0, 1);
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
    };

    let mut stack: Vec<f64> = vec![];
    stack.push(5.0);
    let mut cmd: Vec<&str> = vec![];
    cmd.push("sqr");
    operations::sqr(&mut stack, cmd, &mut vars, 0, 1);
    assert_f64(stack[1], 25.0, statement);

    // sqr lx
    stack = vec![];
    statement = "sqr lx";
    vars.lx = 3.0;
    cmd = statement.split(" ").collect();
    operations::sqr(&mut stack, cmd, &mut vars, 0, 1);
    assert_f64(vars.lx, 9.0, statement);

    // sqr rv
    stack = vec![];
    statement = "sqr rv";
    vars.rv = 4.0;
    cmd = statement.split(" ").collect();
    operations::sqr(&mut stack, cmd, &mut vars, 0, 1);
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
    };

    let mut stack: Vec<f64> = vec![];
    stack.push(9.0);
    let mut cmd: Vec<&str> = vec![];
    cmd.push("sqrt");
    operations::sqrt(&mut stack, cmd, &mut vars, 0, 1);
    assert_f64(stack[1], 3.0, statement);

    // sqr lx
    stack = vec![];
    statement = "sqrt lx";
    vars.lx = 25.0;
    cmd = statement.split(" ").collect();
    operations::sqrt(&mut stack, cmd, &mut vars, 0, 1);
    assert_f64(vars.lx, 5.0, statement);

    // sqr rv
    stack = vec![];
    statement = "sqrt rv";
    vars.rv = 4.0;
    cmd = statement.split(" ").collect();
    operations::sqrt(&mut stack, cmd, &mut vars, 0, 1);
    assert_f64(vars.rv, 2.0, statement);
}
