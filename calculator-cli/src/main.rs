use clap::{App, Arg, crate_version};
use std::io::{Write, stdin, stdout};
use colored::Colorize;
use calculator_parser::calculate;

fn main() {
    let mut app = App::new("Calculator")
    .version(crate_version!())
    .author("AsyncBanana")
    .arg(
        Arg::with_name("Input")
            .short("i")
            .long("input")
            .help("The input")
            .required(false)
            .value_name("INPUT"),
    );
    match app.clone().get_matches().value_of("Input") {
        None => {
            // Launch into repl
            println!("Welcome to calculator version {}. For help, type help",crate_version!());
            loop {
                print!("> ");
                stdout().flush().unwrap();
                let mut input = String::new();
                let _ = stdin().read_line(&mut input);
                input = input.trim().to_string();
                if input == "help" {
                    let _ = app.print_long_help();
                    println!("");
                } else {
                    println!("{}",format!("Result: {:.}",calculate(&input).unwrap()).green().bold())
                }
            }
        }
        Some(input) => {
                println!("{}",format!("Result: {:.}", calculate(input).unwrap()).green().bold());
        }
    }
}
