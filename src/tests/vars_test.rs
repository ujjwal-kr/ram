use super::super::funcs::var;
use super::*;
use std::collections::HashMap;

pub fn global_var_works() {
    let mut hash_vars = super::super::HashVars {
        hash_str: HashMap::new(),
        hash_int: HashMap::new(),
        hash_int_vec: HashMap::new(),
        hash_str_vec: HashMap::new(),
    };

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
        var_int_vec: HashMap::new(),
    };

    // global_var x str >> something
    let mut statement = "global_var x str >> something";
    let mut cmd: Vec<&str> = statement.split(" ").collect();
    let mut stack: Vec<f64> = vec![];

    var::global_var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    match hash_vars.hash_str.get("x") {
        Some(value) => assert_str(value.trim(), "something", statement),
        _ => assert_str("fail", "something", statement),
    }

    // global_var y int >> 50
    statement = "global_var y int >> 50";
    cmd = statement.split(" ").collect();
    var::global_var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    match hash_vars.hash_int.get("y") {
        Some(&value) => assert_f64(value, 50.0, statement),
        _ => assert_str("fail", "something", statement),
    }

    // global_var w int vec >> [10, 13, 12]
    statement = "global_var w int vec >> [10, 13]";
    cmd = statement.split(" ").collect();
    var::global_var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    match hash_vars.hash_int_vec.get("w") {
        Some(value) => assert_vec_int(value.to_vec(), [10.0, 13.0].to_vec(), statement),
        _ => assert_str("fail", "something", statement),
    }

    // global_var v str vec >> [one,two]
    statement = "global_var v str vec >> [one,two]";
    cmd = statement.split(" ").collect();
    var::global_var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    match hash_vars.hash_str_vec.get("v") {
        Some(value) => assert_vec_str(value.to_vec(), ["one", "two"].to_vec(), statement),
        _ => assert_str("fail", "something", statement),
    }

    vars.lx = 1.0;
    vars.rv = 2.0;
    hash_vars.hash_int.insert("name".to_string(), 3.0);
    hash_vars
        .hash_int_vec
        .insert("test".to_string(), [0.0].to_vec());

    statement = "global_var test int vec push >> lx";
    cmd = statement.split(" ").collect();
    var::global_var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    match hash_vars.hash_int_vec.get("test") {
        Some(value) => assert_f64(value.to_vec()[value.to_vec().len() - 1], vars.lx, statement),
        _ => assert_str("fail", "something", statement),
    }

    statement = "global_var test int vec push >> rv";
    cmd = statement.split(" ").collect();
    var::global_var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    match hash_vars.hash_int_vec.get("test") {
        Some(value) => assert_f64(value.to_vec()[value.to_vec().len() - 1], vars.rv, statement),
        _ => assert_str("fail", "something", statement),
    }

    statement = "global_var test int vec push >> global_var name";
    cmd = statement.split(" ").collect();
    var::global_var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    match hash_vars.hash_int_vec.get("test") {
        Some(value) => assert_f64(value.to_vec()[value.to_vec().len() - 1], 3.0, statement),
        _ => assert_str("fail", "something", statement),
    }

    vars.string = "h".to_string();
    vars.lxstring = "e".to_string();
    hash_vars
        .hash_str
        .insert("name".to_string(), "ll".to_string());
    hash_vars
        .hash_str_vec
        .insert("test".to_string(), ["".to_string()].to_vec());

    statement = "global_var test str vec push >> string";
    cmd = statement.split(" ").collect();
    var::global_var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    match hash_vars.hash_str_vec.get("test") {
        Some(value) => assert_str(
            value.to_vec()[value.to_vec().len() - 1].trim(),
            vars.string.trim(),
            statement,
        ),
        _ => assert_str("fail", "something", statement),
    }

    statement = "global_var test str vec push >> lxstring";
    cmd = statement.split(" ").collect();
    var::global_var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    match hash_vars.hash_str_vec.get("test") {
        Some(value) => assert_str(
            value.to_vec()[value.to_vec().len() - 1].trim(),
            vars.lxstring.trim(),
            statement,
        ),
        _ => assert_str("fail", "something", statement),
    }

    statement = "global_var test str vec push >> global_var name";
    cmd = statement.split(" ").collect();
    var::global_var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    match hash_vars.hash_str_vec.get("test") {
        Some(value) => assert_str(
            value.to_vec()[value.to_vec().len() - 1].trim(),
            "ll",
            statement,
        ),
        _ => assert_str("fail", "something", statement),
    }

    hash_vars
        .hash_int_vec
        .insert("test".to_string(), [1.0, 1.1, 2.1].to_vec());
    hash_vars.hash_int.insert("name".to_string(), 0.0);

    statement = "global_var test int vec lx >> [1]";
    cmd = statement.split(" ").collect();
    var::global_var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    assert_f64(vars.lx, 1.1, statement);

    statement = "global_var test int vec rv >> [0]";
    cmd = statement.split(" ").collect();
    var::global_var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    assert_f64(vars.rv, 1.0, statement);

    statement = "global_var test int vec global_var name >> [2]";
    cmd = statement.split(" ").collect();
    var::global_var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    match hash_vars.hash_int.get("name") {
        Some(&value) => assert_f64(value, 2.1, statement),
        _ => assert_str("fail", "something", statement),
    }

    hash_vars.hash_str_vec.insert(
        "test".to_string(),
        ["r".to_string(), "a".to_string(), "m".to_string()].to_vec(),
    );
    hash_vars
        .hash_str
        .insert("name".to_string(), "".to_string());

    statement = "global_var test str vec string >> [0]";
    cmd = statement.split(" ").collect();
    var::global_var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    assert_str(vars.string.trim(), "r", statement);

    statement = "global_var test str vec lxstring >> [1]";
    cmd = statement.split(" ").collect();
    var::global_var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    assert_str(vars.lxstring.trim(), "a", statement);

    statement = "global_var test str vec global_var name >> [2]";
    cmd = statement.split(" ").collect();
    var::global_var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    match hash_vars.hash_str.get("name") {
        Some(value) => assert_str(value.trim(), "m", statement),
        _ => assert_str("fail", "something", statement),
    }

    statement = "global_var test int vec >> [1,3, 12, 14]";
    cmd = statement.split(" ").collect();
    var::global_var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );

    statement = "global_var test int vec len";
    cmd = statement.split(" ").collect();
    var::global_var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    if stack.len() >= 1 {
        assert_f64(stack[stack.len() - 1], 4.0, statement);
    } else {
        assert_str("fail", "something", statement);
    }

    statement = "global_var test str vec >> [one,two, three]";
    cmd = statement.split(" ").collect();
    var::global_var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );

    statement = "global_var test str vec len";
    cmd = statement.split(" ").collect();
    var::global_var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    if stack.len() >= 1 {
        assert_f64(stack[stack.len() - 1], 3.0, statement);
    } else {
        assert_str("fail", "something", statement);
    }
}

pub fn var_works() {
    let mut hash_vars = super::super::HashVars {
        hash_str: HashMap::new(),
        hash_int: HashMap::new(),
        hash_int_vec: HashMap::new(),
        hash_str_vec: HashMap::new(),
    };

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
        var_int_vec: HashMap::new(),
    };

    // var x str >> something
    let mut statement = "var x str >> something";
    let mut cmd: Vec<&str> = statement.split(" ").collect();
    let mut stack: Vec<f64> = vec![];

    var::var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    match vars.var_str.get("x") {
        Some(value) => assert_str(value.trim(), "something", statement),
        _ => assert_str("fail", "something", statement),
    }

    // var y int >> 50
    statement = "var y int >> 50";
    cmd = statement.split(" ").collect();
    var::var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    match vars.var_int.get("y") {
        Some(&value) => assert_f64(value, 50.0, statement),
        _ => assert_str("fail", "something", statement),
    }

    // var w int vec >> [10, 13, 12]
    statement = "var w int vec >> [10, 13]";
    cmd = statement.split(" ").collect();
    var::var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    match vars.var_int_vec.get("w") {
        Some(value) => assert_vec_int(value.to_vec(), [10.0, 13.0].to_vec(), statement),
        _ => assert_str("fail", "something", statement),
    }

    // var v str vec >> [one,two]
    statement = "var v str vec >> [one,two]";
    cmd = statement.split(" ").collect();
    var::var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    match vars.var_str_vec.get("v") {
        Some(value) => assert_vec_str(value.to_vec(), ["one", "two"].to_vec(), statement),
        _ => assert_str("fail", "something", statement),
    }

    vars.lx = 1.0;
    vars.rv = 2.0;
    vars.var_int.insert("name".to_string(), 3.0);
    vars.var_int_vec.insert("test".to_string(), [0.0].to_vec());

    statement = "var test int vec push >> lx";
    cmd = statement.split(" ").collect();
    var::var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    match vars.var_int_vec.get("test") {
        Some(value) => assert_f64(value.to_vec()[value.to_vec().len() - 1], vars.lx, statement),
        _ => assert_str("fail", "something", statement),
    }

    statement = "var test int vec push >> rv";
    cmd = statement.split(" ").collect();
    var::var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    match vars.var_int_vec.get("test") {
        Some(value) => assert_f64(value.to_vec()[value.to_vec().len() - 1], vars.rv, statement),
        _ => assert_str("fail", "something", statement),
    }

    statement = "var test int vec push >> var name";
    cmd = statement.split(" ").collect();
    var::var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    match vars.var_int_vec.get("test") {
        Some(value) => assert_f64(value.to_vec()[value.to_vec().len() - 1], 3.0, statement),
        _ => assert_str("fail", "something", statement),
    }

    vars.string = "h".to_string();
    vars.lxstring = "e".to_string();
    vars.var_str.insert("name".to_string(), "ll".to_string());
    vars.var_str_vec
        .insert("test".to_string(), ["".to_string()].to_vec());

    statement = "var test str vec push >> string";
    cmd = statement.split(" ").collect();
    var::var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    match vars.var_str_vec.get("test") {
        Some(value) => assert_str(
            value.to_vec()[value.to_vec().len() - 1].trim(),
            vars.string.trim(),
            statement,
        ),
        _ => assert_str("fail", "something", statement),
    }

    statement = "var test str vec push >> lxstring";
    cmd = statement.split(" ").collect();
    var::var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    match vars.var_str_vec.get("test") {
        Some(value) => assert_str(
            value.to_vec()[value.to_vec().len() - 1].trim(),
            vars.lxstring.trim(),
            statement,
        ),
        _ => assert_str("fail", "something", statement),
    }

    statement = "var test str vec push >> var name";
    cmd = statement.split(" ").collect();
    var::var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    match vars.var_str_vec.get("test") {
        Some(value) => assert_str(
            value.to_vec()[value.to_vec().len() - 1].trim(),
            "ll",
            statement,
        ),
        _ => assert_str("fail", "something", statement),
    }

    vars.var_int_vec
        .insert("test".to_string(), [1.0, 1.1, 2.1].to_vec());
    vars.var_int.insert("name".to_string(), 0.0);

    statement = "var test int vec lx >> [1]";
    cmd = statement.split(" ").collect();
    var::var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    assert_f64(vars.lx, 1.1, statement);

    statement = "var test int vec rv >> [0]";
    cmd = statement.split(" ").collect();
    var::var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    assert_f64(vars.rv, 1.0, statement);

    statement = "var test int vec var name >> [2]";
    cmd = statement.split(" ").collect();
    var::var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    match vars.var_int.get("name") {
        Some(&value) => assert_f64(value, 2.1, statement),
        _ => assert_str("fail", "something", statement),
    }

    vars.var_str_vec.insert(
        "test".to_string(),
        ["r".to_string(), "a".to_string(), "m".to_string()].to_vec(),
    );
    hash_vars
        .hash_str
        .insert("name".to_string(), "".to_string());

    statement = "var test str vec string >> [0]";
    cmd = statement.split(" ").collect();
    var::var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    assert_str(vars.string.trim(), "r", statement);

    statement = "var test str vec lxstring >> [1]";
    cmd = statement.split(" ").collect();
    var::var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    assert_str(vars.lxstring.trim(), "a", statement);

    statement = "var test str vec var name >> [2]";
    cmd = statement.split(" ").collect();
    var::var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    match vars.var_str.get("name") {
        Some(value) => assert_str(value.trim(), "m", statement),
        _ => assert_str("fail", "something", statement),
    }

    statement = "var test int vec >> [1,3, 12, 14]";
    cmd = statement.split(" ").collect();
    var::var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );

    statement = "var test int vec len";
    cmd = statement.split(" ").collect();
    var::var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    if stack.len() >= 1 {
        assert_f64(stack[stack.len() - 1], 4.0, statement);
    } else {
        assert_str("fail", "something", statement);
    }

    statement = "var test str vec >> [one,two, three]";
    cmd = statement.split(" ").collect();
    var::global_var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );

    statement = "var test str vec len";
    cmd = statement.split(" ").collect();
    var::var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );
    if stack.len() >= 1 {
        assert_f64(stack[stack.len() - 1], 3.0, statement);
    } else {
        assert_str("fail", "something", statement);
    }
}

pub fn move_works() {
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
        var_int_vec: HashMap::new(),
    };

    let mut hash_vars = super::super::HashVars {
        hash_str: HashMap::new(),
        hash_int: HashMap::new(),
        hash_int_vec: HashMap::new(),
        hash_str_vec: HashMap::new(),
    };

    let mut stack: Vec<f64> = vec![];

    // move int lx = var <name>

    let mut statement: &str = "global_var x int >> 40";
    let mut cmd: Vec<&str> = statement.split(" ").collect();
    var::global_var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );

    statement = "move int lx = global_var x";
    cmd = statement.split(" ").collect();

    var::movefn(cmd, &mut vars, &mut hash_vars, "0", 1);
    match hash_vars.hash_int.get("x") {
        Some(&value) => assert_f64(value, vars.lx, statement),
        _ => assert_str("fail", "something", statement),
    }

    // move int rv = var y
    let mut statement: &str = "global_var y int >> 30";
    let mut cmd: Vec<&str> = statement.split(" ").collect();
    var::global_var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );

    statement = "move int rv = global_var y";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, &mut hash_vars, "0", 1);
    match hash_vars.hash_int.get("y") {
        Some(&value) => assert_f64(value, vars.rv, statement),
        _ => assert_str("fail", "something", statement),
    }

    // move str string  = var z
    statement = "global_var z str >> ok";
    cmd = statement.split(" ").collect();
    var::global_var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );

    statement = "move str string = global_var z";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, &mut hash_vars, "0", 1);
    match hash_vars.hash_str.get("z") {
        Some(value) => assert_str(value.trim(), vars.string.trim(), statement),
        _ => assert_str("fail", "something", statement),
    }

    // move str lxstring = var z
    statement = "global_var z str >> ok2";
    cmd = statement.split(" ").collect();
    var::global_var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );

    statement = "move str lxstring = global_var z";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, &mut hash_vars, "0", 1);
    match hash_vars.hash_str.get("z") {
        Some(value) => assert_str(value.trim(), vars.lxstring.trim(), statement),
        _ => assert_str("fail", "something", statement),
    }

    // move str var exm = string
    vars.string = "hello".to_string();
    statement = "move str global_var exm = string";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, &mut hash_vars, "0", 1);
    match hash_vars.hash_str.get("exm") {
        Some(value) => assert_str(value.trim(), vars.string.trim(), statement),
        _ => assert_str("fail", "something", statement),
    }

    // move str var lxstr = lxstring
    vars.lxstring = "hello2".to_string();
    statement = "move str global_var lxstr = lxstring";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, &mut hash_vars, "0", 1);
    match hash_vars.hash_str.get("lxstr") {
        Some(value) => assert_str(value.trim(), vars.lxstring.trim(), statement),
        _ => assert_str("fail", "something", statement),
    }

    // move int global_var testint = lx
    vars.lx = 15.0;
    statement = "move int global_var testint = lx";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, &mut hash_vars, "0", 1);
    match hash_vars.hash_int.get("testint") {
        Some(&value) => assert_f64(value, vars.lx, statement),
        _ => assert_str("fail", "something", statement),
    }

    // move int var testint = rv
    vars.rv = 16.0;
    statement = "move int global_var testint = rv";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, &mut hash_vars, "0", 1);
    match hash_vars.hash_int.get("testint") {
        Some(&value) => assert_f64(value, vars.rv, statement),
        _ => assert_str("fail", "something", statement),
    }

    // move vec vec str = var test

    statement = "global_var test str vec >> [t, one]";
    cmd = statement.split(" ").collect();
    var::global_var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );

    statement = "move vec vec str = global_var test";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, &mut hash_vars, "0", 1);
    match hash_vars.hash_str_vec.get("test") {
        Some(value) => assert_vec_string(value.to_vec(), vars.str_vec.clone(), statement),
        _ => assert_str("fail", "something", statement),
    }

    // move vec var test2 = vec str
    vars.str_vec = ["one".to_string(), "two".to_string()].to_vec();
    statement = "move vec global_var test2 = vec str";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, &mut hash_vars, "0", 1);
    match hash_vars.hash_str_vec.get("test2") {
        Some(value) => assert_vec_string(value.to_vec(), vars.str_vec.clone(), statement),
        _ => assert_str("fail", "something", statement),
    }

    // move vec vec int = var test3

    statement = "global_var test3 int vec >> [1.0, 3.0]";
    cmd = statement.split(" ").collect();
    var::global_var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );

    statement = "move vec vec int = global_var test3";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, &mut hash_vars, "0", 1);
    match hash_vars.hash_int_vec.get("test3") {
        Some(value) => assert_vec_int(value.to_vec(), vars.num_vec.clone(), statement),
        _ => assert_str("fail", "something", statement),
    }

    // move vec var test4 = vec int
    vars.num_vec = [1.6, 2.7].to_vec();
    statement = "move vec global_var test4 = vec int";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, &mut hash_vars, "0", 1);
    match hash_vars.hash_int_vec.get("test4") {
        Some(value) => assert_vec_int(value.to_vec(), vars.num_vec, statement),
        _ => assert_str("fail", "something", statement),
    }
}

pub fn local_move_works() {
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
        var_int_vec: HashMap::new(),
    };

    let mut hash_vars = super::super::HashVars {
        hash_str: HashMap::new(),
        hash_int: HashMap::new(),
        hash_int_vec: HashMap::new(),
        hash_str_vec: HashMap::new(),
    };

    let mut stack: Vec<f64> = vec![];

    // move int lx = var <name>

    let mut statement: &str = "var x int >> 40";
    let mut cmd: Vec<&str> = statement.split(" ").collect();
    var::var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );

    statement = "move int lx = var x";
    cmd = statement.split(" ").collect();

    var::movefn(cmd, &mut vars, &mut hash_vars, "0", 1);
    match vars.var_int.get("x") {
        Some(&value) => assert_f64(value, vars.lx, statement),
        _ => assert_str("fail", "something", statement),
    }

    // move int rv = var y
    let mut statement: &str = "var y int >> 30";
    let mut cmd: Vec<&str> = statement.split(" ").collect();
    var::var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );

    statement = "move int rv = var y";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, &mut hash_vars, "0", 1);
    match vars.var_int.get("y") {
        Some(&value) => assert_f64(value, vars.rv, statement),
        _ => assert_str("fail", "something", statement),
    }

    // move str string  = var z
    statement = "var z str >> ok";
    cmd = statement.split(" ").collect();
    var::var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );

    statement = "move str string = var z";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, &mut hash_vars, "0", 1);
    match vars.var_str.get("z") {
        Some(value) => assert_str(value.trim(), vars.string.trim(), statement),
        _ => assert_str("fail", "something", statement),
    }

    // move str lxstring = var z
    statement = "var z str >> ok2";
    cmd = statement.split(" ").collect();
    var::var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );

    statement = "move str lxstring = var z";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, &mut hash_vars, "0", 1);
    match vars.var_str.get("z") {
        Some(value) => assert_str(value.trim(), vars.lxstring.trim(), statement),
        _ => assert_str("fail", "something", statement),
    }

    // move str var exm = string
    vars.string = "hello".to_string();
    statement = "move str var exm = string";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, &mut hash_vars, "0", 1);
    match vars.var_str.get("exm") {
        Some(value) => assert_str(value.trim(), vars.string.trim(), statement),
        _ => assert_str("fail", "something", statement),
    }

    // move str var lxstr = lxstring
    vars.lxstring = "hello2".to_string();
    statement = "move str var lxstr = lxstring";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, &mut hash_vars, "0", 1);
    match vars.var_str.get("lxstr") {
        Some(value) => assert_str(value.trim(), vars.lxstring.trim(), statement),
        _ => assert_str("fail", "something", statement),
    }

    // move int var testint = lx
    vars.lx = 15.0;
    statement = "move int var testint = lx";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, &mut hash_vars, "0", 1);
    match vars.var_int.get("testint") {
        Some(&value) => assert_f64(value, vars.lx, statement),
        _ => assert_str("fail", "something", statement),
    }

    // move int var testint = rv
    vars.rv = 16.0;
    statement = "move int var testint = rv";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, &mut hash_vars, "0", 1);
    match vars.var_int.get("testint") {
        Some(&value) => assert_f64(value, vars.rv, statement),
        _ => assert_str("fail", "something", statement),
    }

    // move vec vec str = var test

    statement = "var test str vec >> [t, one]";
    cmd = statement.split(" ").collect();
    var::var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );

    statement = "move vec vec str = var test";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, &mut hash_vars, "0", 1);
    match vars.var_str_vec.get("test") {
        Some(value) => assert_vec_string(value.to_vec(), vars.str_vec.clone(), statement),
        _ => assert_str("fail", "something", statement),
    }

    // move vec var test2 = vec str
    vars.str_vec = ["one".to_string(), "two".to_string()].to_vec();
    statement = "move vec var test2 = vec str";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, &mut hash_vars, "0", 1);
    match vars.var_str_vec.get("test2") {
        Some(value) => assert_vec_string(value.to_vec(), vars.str_vec.clone(), statement),
        _ => assert_str("fail", "something", statement),
    }

    // move vec vec int = var test3

    statement = "var test3 int vec >> [1.0, 3.0]";
    cmd = statement.split(" ").collect();
    var::var(
        &mut stack,
        cmd,
        statement,
        &mut vars,
        &mut hash_vars,
        "0",
        1,
    );

    statement = "move vec vec int = var test3";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, &mut hash_vars, "0", 1);
    match vars.var_int_vec.get("test3") {
        Some(value) => assert_vec_int(value.to_vec(), vars.num_vec.clone(), statement),
        _ => assert_str("fail", "something", statement),
    }

    // move vec var test4 = vec int
    vars.num_vec = [1.6, 2.7].to_vec();
    statement = "move vec var test4 = vec int";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, &mut hash_vars, "0", 1);
    match vars.var_int_vec.get("test4") {
        Some(value) => assert_vec_int(value.to_vec(), vars.num_vec, statement),
        _ => assert_str("fail", "something", statement),
    }
}
