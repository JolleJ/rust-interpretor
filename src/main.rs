
mod lexer;
mod token;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use lexer::Lexer;
    use token::{Token, TokenType};

    use super::*;

    

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";

        let mut lexer = Lexer::new(input.to_string());
    
        let tests: [Token; 11] = [
            Token { token_type: TokenType::Let, literal: "let".to_string()},
            Token { token_type: TokenType::Ident, literal: "five".to_string()},
            Token { token_type: TokenType::Assign, literal: "=".to_string()},
            Token { token_type: TokenType::Plus, literal: "+".to_string()},
            Token { token_type: TokenType::LParen, literal: "(".to_string()},
            Token { token_type: TokenType::RParen, literal: ")".to_string()},
            Token { token_type: TokenType::LBrace, literal: "{".to_string()},
            Token { token_type: TokenType::RBrace, literal: "}".to_string()},
            Token { token_type: TokenType::Comma, literal: ",".to_string()},
            Token { token_type: TokenType::Semicolon, literal: ";".to_string()},
            Token { token_type: TokenType::Eof, literal: "".to_string()},
            ];

        for (i, tt) in tests.iter().enumerate() {         
            let tok = lexer.next_token();
            assert_eq!(*tt, tok, "token types are not equal");
        }
    
    }
}
