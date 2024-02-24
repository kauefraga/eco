use std::collections::VecDeque;
use std::env;
use std::process::exit;

fn main() {
    let mut args: VecDeque<String> = env::args().collect();

    args.pop_front();

    let is_help_needed = (args.len() == 1 && args[0] == "-h")
        || (args.len() == 1 && args[0] == "--help");
    if is_help_needed {
        println!("Example: eco-rs Bom dia!");
        exit(0);
    }

    let mut output = String::new();

    for arg in args {
        output.push_str(&format!("{arg} "));
    }

    println!("{}", output.trim());
}
