use colored::Colorize;
use my_first_compiler::lexer_base::lexer;
use my_first_compiler::lexer_base::token::Token;
use my_first_compiler::parser_base::parser;
use std::fs;

fn test_lexer(input_path: &str) -> Result<Vec<Token>, String> {
    println!("Lexing file: {}", input_path.green());
    let input = fs::read_to_string(input_path).expect("failed to load test input");

    return match lexer::tokenizer(&input) {
        Err(e) => {
            println!("Lexer Error: {}", e.red());
            Err(e)
        }
        Ok(tokens) => {
            for token in &tokens {
                println!("{:?}", token);
            }
            Ok(tokens)
        }
        
    };
}

#[allow(unused_variables)]
fn test_parser(lexer_result: Vec<Token>) -> Result<(), String> { 
    println!("Parsing tokens...");
    return match parser::parse(&lexer_result) {
        Err(e) => {
            println!("Parser Error: {}", e.red());
            Err(e)
        }
        Ok(ast) => {
            println!("Parsed AST: {:?}", ast);
            Ok(())
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
        if let Ok(tokens) = test_lexer(path) {
            if let Ok(_) = test_parser(tokens) {
                todo!()
            }
        }
    }
}
