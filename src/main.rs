use std::env::args;

fn main() {
    let args: Vec<String> = args().skip(1).collect();

    echo(args);
}

fn echo(args: Vec<String>) {
    if args.len() < 1 {
        panic!("Must be at list one arg to echo it");
    };

    let Some(echo_value) = args.iter().find(|&a| !a.starts_with("-")) else {
        panic!("Nothing to echo");
    };
    let use_special_symbols= args.iter().any(|a| a == "-e");
    let not_to_append_new_line = args.iter().any(|a| a == "-n");
    let mut to_print = echo_value.to_string();

    if use_special_symbols {
        to_print = replace_special_symbols(echo_value);
    };

    if not_to_append_new_line {
        print!("echo: {to_print}")
    } else {
        println!("echo: {to_print}");
    }
}

fn replace_special_symbols(input: &String) -> String {
    input.replace(r"\n", "\n")
    .replace(r"\t", "\t")
    .replace(r"\\", "\\")
    .replace(r"\'", "\'")
    .replace(r#"\""#, "\"")
    .replace(r"\r", "\r")
}
