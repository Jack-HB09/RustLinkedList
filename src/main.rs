pub mod parser;

use std::any::type_name;
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
#[derive(Debug)]
pub enum Token {
    LeftBrace,       // {
    RightBrace,      // }
    Colon,           // :
    Comma,           // ,
    String(String),  // "key" or "value"
    Number(f64),     // 123 or 45.67
    Boolean(bool),   // true or false
    Null,            // null or None
}
pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            // Ignore whitespace
            ' ' | '\t' | '\r' | '\n' => {
                chars.next();
            }
            // Structural punctuation
            '{' => { tokens.push(Token::LeftBrace); chars.next(); }
            '}' => { tokens.push(Token::RightBrace); chars.next(); }
            ':' => { tokens.push(Token::Colon); chars.next(); }
            ',' => { tokens.push(Token::Comma); chars.next(); }

            // Strings
            '"' => {
                chars.next(); // consume opening quote
                let mut string_val = String::new();
                while let Some(&c) = chars.peek() {
                    if c == '"' { break; }
                    string_val.push(c);
                    chars.next();
                }
                chars.next(); // consume closing quote
                tokens.push(Token::String(string_val));
            }

            // Numbers
            '0'..='9' | '-' => {
                let mut num_str = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() || c == '.' || c == '-' {
                        num_str.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let num: f64 = num_str.parse().map_err(|_| "Invalid number")?;
                tokens.push(Token::Number(num));
            }

            // Identifiers (booleans / null)
            'a'..='z' | 'A'..='Z' => {
                let mut ident = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() {
                        ident.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                match ident.as_str() {
                    "true" | "True" => tokens.push(Token::Boolean(true)),
                    "false" | "False" => tokens.push(Token::Boolean(false)),
                    "null" | "None" => tokens.push(Token::Null),
                    _ => return Err(format!("Unknown identifier: {}", ident)),
                }
            }

            _ => return Err(format!("Unexpected character: {}", ch)),
        }
    }

    Ok(tokens)
}
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read your custom hashmap file (e.g., config.jk)
    let file_path = "src/data.jk";
    let source_code = fs::read_to_string(file_path)?;

    // Run tokenizer
    match tokenize(&source_code) {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token);
            }

        }
        Err(err) => eprintln!("Lexing error: {}", err),
    }
    Ok(())
}