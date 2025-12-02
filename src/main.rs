use colored::Colorize;
use my_first_compiler::lexer_base::lexer;
use my_first_compiler::lexer_base::token::Token;
use my_first_compiler::parser_base::parser;

use std::fs;
use walkdir::WalkDir;

fn test_lexer(input_path: &str) -> Result<Vec<Token>, String> {
    println!("Lexing file: {}", input_path.green());
    let input = fs::read_to_string(input_path).expect("failed to load test input");

    return match lexer::tokenizer(&input) {
        Err(e) => {
            println!("Lexer Error: {}", e.red());
            Err(e)
        }
        Ok(tokens) => {
            println!("Lexing passed.");
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
    let test_root = "tests/Ch1";
    for entry in WalkDir::new(test_root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file() && e.path().extension().map_or(false, |ext| ext == "c"))
    {
        let path_str = entry.path().to_str().unwrap();
        println!("==============================");
        println!("Testing file: {}", path_str.blue().bold());
        match test_lexer(path_str) {
            Ok(tokens) => {
                let _ = test_parser(tokens);
            }
            Err(_) => {
                // Lexer error already printed
            }
        }
        println!("==============================\n");
    }
}
