use std::fs;
use colored::Colorize;
pub mod lexer;

fn test_lexer(input_path: &str) {
    println!("Lexing file: {}", input_path.green());
    let input = fs::read_to_string(input_path)
        .expect("failed to load test input");

    match lexer::tokenizer(&input) {
        Err(e) => {
            println!("Lexer Error: {}", e.red());
            return;
        }
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token);
            }
        }
    };
}

fn main() {
    let lexer_input_paths = vec![
        "tests/Ch1/valid/return_0.c",
        "tests/Ch1/valid/return_2.c",
        "tests/Ch1/valid/spaces.c",
        "tests/Ch1/valid/multi_digit.c",
        "tests/Ch1/invalid/at_sign.c",
        "tests/Ch1/invalid/backslash.c",
        "tests/Ch1/invalid/invalid_identifier.c",
        "tests/Ch1/invalid/invalid_identifier_2.c",
    ];
    for path in lexer_input_paths {
        test_lexer(path);
    }
}
