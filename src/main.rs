use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    /*
     * Read the `remove` docstring.
     * This `remove(0)` is the worst case,
     * all the arguments are going to be
     * shifted to the left, everytime.
     */
    args.remove(0);

    let mut output = String::new();

    for arg in args {
        output.push_str(&format!("{arg} "));
    }

    println!("{}", output.trim());
}
