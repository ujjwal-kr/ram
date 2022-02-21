use super::super::funcs::var;
use super::*;
use std::collections::HashMap;

pub fn var_works() {
    // var x str >> something
    let mut statement = "var x str >> something";
    let mut cmd: Vec<&str> = statement.split(" ").collect();

    let mut hash_vars = super::super::HashVars {
        hash_str: HashMap::new(),
        hash_int: HashMap::new(),
        hash_int_vec: HashMap::new(),
        hash_str_vec: HashMap::new(),
    };

    var::var(cmd, statement, &mut hash_vars, 0, 1);
    match hash_vars.hash_str.get("x") {
        Some(value) => assert_str(value.trim(), "something", statement),
        _ => assert_str("fail", "something", statement),
    }

    // var y int >> 50
    statement = "var y int >> 50";
    cmd = statement.split(" ").collect();
    var::var(cmd, statement, &mut hash_vars, 0, 1);
    match hash_vars.hash_int.get("y") {
        Some(&value) => assert_f64(value, 50.0, statement),
        _ => assert_str("fail", "something", statement),
    }

    // var w int vec >> [10, 13, 12]
    statement = "var w int vec >> [10, 13]";
    cmd = statement.split(" ").collect();
    var::var(cmd, statement, &mut hash_vars, 0, 1);
    match hash_vars.hash_int_vec.get("w") {
        Some(value) => assert_vec_int(value.to_vec(), [10.0, 13.0].to_vec(), statement),
        _ => assert_str("fail", "something", statement),
    }

    // var v str vec >> [one,two]
    statement = "var v str vec >> [one,two]";
    cmd = statement.split(" ").collect();
    var::var(cmd, statement, &mut hash_vars, 0, 1);
    match hash_vars.hash_str_vec.get("v") {
        Some(value) => assert_vec_str(value.to_vec(), ["one", "two"].to_vec(), statement),
        _ => assert_str("fail", "something", statement),
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
    };

    let mut hash_vars = super::super::HashVars {
        hash_str: HashMap::new(),
        hash_int: HashMap::new(),
        hash_int_vec: HashMap::new(),
        hash_str_vec: HashMap::new(),
    };

    // move int lx x
    let mut statement: &str = "var x int >> 40";
    let mut cmd: Vec<&str> = statement.split(" ").collect();
    var::var(cmd, statement, &mut hash_vars, 0, 1);

    statement = "move int lx x";
    cmd = statement.split(" ").collect();

    var::movefn(cmd, &mut vars, &mut hash_vars, 0, 1);
    match hash_vars.hash_int.get("x") {
        Some(&value) => assert_f64(value, vars.lx, statement),
        _ => assert_str("fail", "something", statement),
    }

    // move int rv y
    let mut statement: &str = "var y int >> 30";
    let mut cmd: Vec<&str> = statement.split(" ").collect();
    var::var(cmd, statement, &mut hash_vars, 0, 1);

    statement = "move int rv y";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, &mut hash_vars, 0, 1);
    match hash_vars.hash_int.get("y") {
        Some(&value) => assert_f64(value, vars.rv, statement),
        _ => assert_str("fail", "something", statement),
    }

    // move str string z
    statement = "var z str >> ok";
    cmd = statement.split(" ").collect();
    var::var(cmd, statement, &mut hash_vars, 0, 1);

    statement = "move str string z";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, &mut hash_vars, 0, 1);
    match hash_vars.hash_str.get("z") {
        Some(value) => assert_str(value.trim(), vars.string.trim(), statement),
        _ => assert_str("fail", "something", statement),
    }

    // move str lxstring z
    statement = "var z str >> ok2";
    cmd = statement.split(" ").collect();
    var::var(cmd, statement, &mut hash_vars, 0, 1);

    statement = "move str lxstring z";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, &mut hash_vars, 0, 1);
    match hash_vars.hash_str.get("z") {
        Some(value) => assert_str(value.trim(), vars.lxstring.trim(), statement),
        _ => assert_str("fail", "something", statement),
    }

    // move str var exm string
    vars.string = "hello".to_string();
    statement = "move str var exm string";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, &mut hash_vars, 0, 1);
    match hash_vars.hash_str.get("exm") {
        Some(value) => assert_str(value.trim(), vars.string.trim(), statement),
        _ => assert_str("fail", "something", statement),
    }

    // move str var lxstr lxstring
    vars.lxstring = "hello2".to_string();
    statement = "move str var lxstr lxstring";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, &mut hash_vars, 0, 1);
    match hash_vars.hash_str.get("lxstr") {
        Some(value) => assert_str(value.trim(), vars.lxstring.trim(), statement),
        _ => assert_str("fail", "something", statement),
    }

    // move int var testint lx
    vars.lx = 15.0;
    statement = "move int var testint lx";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, &mut hash_vars, 0, 1);
    match hash_vars.hash_int.get("testint") {
        Some(&value) => assert_f64(value, vars.lx, statement),
        _ => assert_str("fail", "something", statement),
    }

    // move int var testint rv
    vars.rv = 16.0;
    statement = "move int var testint rv";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, &mut hash_vars, 0, 1);
    match hash_vars.hash_int.get("testint") {
        Some(&value) => assert_f64(value, vars.rv, statement),
        _ => assert_str("fail", "something", statement),
    }

    // move vec vec str var test

    statement = "var test str vec >> [t, one]";
    cmd = statement.split(" ").collect();
    var::var(cmd, statement, &mut hash_vars, 0, 1);

    statement = "move vec vec str var test";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, &mut hash_vars, 0, 1);
    match hash_vars.hash_str_vec.get("test") {
        Some(value) => assert_vec_string(value.to_vec(), vars.str_vec.clone(), statement),
        _ => assert_str("fail", "something", statement),
    }

    // move vec var test2 vec str
    vars.str_vec = ["one".to_string(), "two".to_string()].to_vec();
    statement = "move vec var test2 vec str";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, &mut hash_vars, 0, 1);
    match hash_vars.hash_str_vec.get("test2") {
        Some(value) => assert_vec_string(value.to_vec(), vars.str_vec.clone(), statement),
        _ => assert_str("fail", "something", statement),
    }

    // move vec vec int var test3

    statement = "var test3 int vec >> [1.0, 3.0]";
    cmd = statement.split(" ").collect();
    var::var(cmd, statement, &mut hash_vars, 0, 1);

    statement = "move vec vec int var test3";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, &mut hash_vars, 0, 1);
    match hash_vars.hash_int_vec.get("test3") {
        Some(value) => assert_vec_int(value.to_vec(), vars.num_vec.clone(), statement),
        _ => assert_str("fail", "something", statement),
    }

    // move vec var test4 vec int
    vars.num_vec = [1.6, 2.7].to_vec();
    statement = "move vec var test4 vec int";
    cmd = statement.split(" ").collect();
    var::movefn(cmd, &mut vars, &mut hash_vars, 0, 1);
    match hash_vars.hash_int_vec.get("test4") {
        Some(value) => assert_vec_int(value.to_vec(), vars.num_vec.clone(), statement),
        _ => assert_str("fail", "something", statement),
    }
}
