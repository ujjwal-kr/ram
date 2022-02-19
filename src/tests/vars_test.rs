use super::super::funcs::errors;
use super::super::funcs::var;
use super::*;
use std::collections::HashMap;

pub fn var_works() {
    // var x str >> something
    let mut statement = "var x str >> something";
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
    var::var(cmd, statement, &mut vars, 0, 1);
    match vars.hash_str.get(&"x") {
        Some(&value) => assert_str(value, "something", statement),
        _ => assert_str("fail", "something", statement),
    }

    // var y int >> 50
    statement = "var y int >> 50";
    cmd = statement.split(" ").collect();
    var::var(cmd, statement, &mut vars, 0, 1);
    match vars.hash_int.get(&"y") {
        Some(&value) => assert_f64(errors::parse_float(value, 0, 1), 50.0, statement),
        _ => assert_str("fail", "something", statement),
    }

    // var w vec str >> [one, two]
    statement = "var w vec str >> [one, two]";
    cmd = statement.split(" ").collect();
    var::var(cmd, statement, &mut vars, 0, 1);
    match vars.hash_str_vec.get(&"w") {
        Some(value) => assert_var_vec_str((&value).to_vec(), ["one", "two"].to_vec(), statement),
        _ => assert_str("fail", "something", statement),
    }

    // var v vec int >> [1, 2]
    statement = "var v vec int >> [1, 2]";
    cmd = statement.split(" ").collect();
    var::var(cmd, statement, &mut vars, 0, 1);
    match vars.hash_int_vec.get(&"v") {
        Some(value) => assert_vec_int((&value).to_vec(), [1.0, 2.0].to_vec(), statement),
        _ => assert_str("fail", "something", statement),
    }
}
