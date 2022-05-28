use std::process;

pub fn parse_float(arg: &str, run_label: &str, line: u32) -> f64 {
    let num: f64;
    num = arg.trim().parse().expect(
        format!(
            "Input a number at {} line: {}",
            run_label,
            line.to_string()
        )
        .trim(),
    );
    num
}

pub fn stack_len_error(run_label: &str, line: u32) {
    println!(
        "Not enough items in the stack, {} line:{}",
        run_label.to_string(),
        line.to_string()
    );
    process::exit(1)
}

pub fn parse_usize(arg: &str, block: &str, line: u32) -> usize {
    let num: usize;
    num = arg.parse().expect(
        format!(
            "Invalid usize, input a number at {} line: {}",
            block.to_string(),
            line.to_string()
        )
        .trim(),
    );
    num
}

pub fn args_error(run_label: &str, line: u32) {
    println!(
        "Invalid arguments at {} line:{}",
        run_label.to_string(),
        line.to_string()
    );
    process::exit(1)
}

pub fn invalid_index(run_label: &str, line: u32, inv_index: usize) {
    println!(
        "Invalid vector index, position {} does not exist in vec at {} line:{}",
        inv_index.to_string(),
        run_label.to_string(),
        line.to_string()
    );
    process::exit(1)
}

pub fn var_error(var_name: &str, run_label: &str, line: u32) {
    println!(
        "Var {} does not exist at {} line:{}",
        var_name.to_string(),
        run_label.to_string(),
        line.to_string()
    );
    process::exit(1)
}

pub fn vec_items(run_label: &str, line: u32) {
    println!(
        "Not enough items in the vec at {} line:{}",
        run_label,
        line.to_string()
    );
    process::exit(1)
}
