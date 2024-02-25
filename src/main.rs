use std::env;
use std::process::exit;

use console::style;

fn main() {
    let mut output = String::new();
    let mut args_counter = 0;

    for arg in env::args() {
        // Skip the first arg (executable)
        if args_counter == 0 {
            args_counter += 1;
            continue;
        }

        let is_help_needed = env::args().len() == 2
            && (arg == "-h" || arg == "--help");
        if is_help_needed {
            println!("{}", format!(
                "{}: eco-rs bom dia! {}",
                style("Example").green(),
                style("// bom dia!").dim()
            ));
            exit(0);
        }

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
        args_counter += 1;
    }

    println!("{}", output.trim())
}
