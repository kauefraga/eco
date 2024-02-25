use std::collections::VecDeque;
use std::env;
use std::process::exit;

use console::style;

fn main() {
    let mut args: VecDeque<String> = env::args().collect();

    args.pop_front();

    let is_help_needed = args.len() == 1
        && (args[0] == "-h" || args[0] == "--help");
    if is_help_needed {
        println!("{}", format!(
            "{}: eco-rs bom dia! {}",
            style("Example").green(),
            style("// bom dia!").dim()
        ));
        exit(0);
    }

    let mut output = String::new();

    for arg in args {
        if arg.to_lowercase().contains("rust") {
            arg.split(' ').for_each(|word| {
                if word.to_lowercase().contains("rust") {
                    output.push_str(&format!(
                        "{} ",
                        style(word).red()
                    ));
                } else {
                    output.push_str(&format!("{word} "));
                }
            });
            continue;
        }

        output.push_str(&format!("{arg} "));
    }

    println!("{}", output.trim());
}
