use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_length: usize = args.len();

    let modified_args: Vec<String> = args[1..args_length].to_vec();
    let echo_statement: String = modified_args.join(" ");

    println!("{}", echo_statement);

}

