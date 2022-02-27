use colored::*;
use std::fs::File;
use std::io::prelude::*;
pub mod operation_test;
pub mod stack_test;
pub mod vars_test;

use operation_test::*;
use stack_test::*;
use vars_test::*;

pub fn test() {
    test_log("ram");
    ram_works();
    test_log("str");
    str_works();
    test_log("operations");
    operation_works();
    test_log("var");
    var_works();
    test_log("move");
    move_works();
}

pub fn test_log(string: &str) {
    println!("{} {}", "TESTING:".cyan().bold(), string);
}

pub fn log_pass(string: &str) {
    println!("{}: {}", "PASSED".green().bold(), string.blue());
}

pub fn log_fail(string: &str) {
    println!("{}: {}", "FAILED".red().bold(), string.yellow());
    let mut file = File::create("log.txt").expect("err creating log");
    file.write_all(b"true").expect("err creating log");
}

pub fn assert_f64(a: f64, b: f64, statement: &str) {
    if a == b {
        log_pass(statement)
    } else {
        log_fail(statement)
    }
}

pub fn assert_str(a: &str, b: &str, statement: &str) {
    if a == b {
        log_pass(statement)
    } else {
        log_fail(statement)
    }
}

pub fn assert_vec_int(a: Vec<f64>, b: Vec<f64>, statement: &str) {
    if a.len() != b.len() {
        return log_fail(statement);
    }
    let mut i = 0usize;
    for item in a {
        if item != b[i] {
            return log_fail(statement);
        }
        i = i + 1;
    }
    log_pass(statement)
}

pub fn assert_vec_str(a: Vec<String>, b: Vec<&str>, statement: &str) {
    if a.len() != b.len() {
        return log_fail(statement);
    }
    let mut i = 0usize;
    for item in a {
        if item.trim() != b[i] {
            return log_fail(statement);
        }
        i = i + 1;
    }
    log_pass(statement)
}

pub fn assert_vec_string(a: Vec<String>, b: Vec<String>, statement: &str) {
    if a.len() != b.len() {
        return log_fail(statement);
    }
    let mut i = 0usize;
    for item in a {
        if item.trim() != b[i].trim() {
            return log_fail(statement);
        }
        i = i + 1;
    }
    log_pass(statement)
}
