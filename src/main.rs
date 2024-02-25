use std::env;
use std::process::exit;

use console::style;

fn main() {
    let mut output = String::new();

    let mut args = env::args();
    args.next(); // skip first argument (executable)

    match args.next() {
        Some(a) => {
            let is_help_needed = env::args().len() == 2
                && (a == "-h" || a == "--help");
            if is_help_needed {
                println!("{}", format!(
                    "{}: eco-rs bom dia! {}",
                    style("Example").green(),
                    style("// bom dia!").dim()
                ));
                exit(0);
            }

            output.push_str(&format!("{a} "));
        },
        None => {
            println!("");
            exit(0);
        }
    }

    while let Some(arg) = args.next() {
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

    println!("{}", output.trim())
}
