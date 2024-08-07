use std::char;

use crate::token::{Token, TokenType};



pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };

        lexer.read_char();

        return lexer;
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }

        self.position = self.read_position;
        self.read_position += 1; 
    }

    pub fn next_token(&mut self) -> Token {
        match self.ch {
            '=' => {
                let token = Token {
                    token_type: TokenType::Assign,
                    literal: self.ch.to_string(),
                };
                self.read_char();
                return token;
            },
            ';' => {
                let token = Token::new(TokenType::Semicolon, self.ch.to_string());
                self.read_char();
                return token;
            },
            '(' => {
                let token = Token::new(TokenType::LParen, self.ch.to_string());
                self.read_char();
                return token;
            },
            ')' => {
                let token = Token::new(TokenType::RParen, self.ch.to_string());
                self.read_char();
                return token;
            },
            ',' => {
                let token = Token::new(TokenType::Comma, self.ch.to_string());
                self.read_char();
                return token;
            },
            '+' => {
                let token = Token::new(TokenType::Plus, self.ch.to_string());
                self.read_char();
                return token;
            },
            '{' => {
                let token = Token::new(TokenType::LBrace, self.ch.to_string());
                self.read_char();
                return token;
            },
            '}' => {
                let token = Token::new(TokenType::RBrace, self.ch.to_string());
                self.read_char();
                return token;
            },
            '\0' => {
                let token = Token::new(TokenType::Eof, "".to_string());
                return token;
            },
            _ => {
                let token = Token {
                    token_type: TokenType::Illegal,
                    literal: self.ch.to_string(),
                };
                self.read_char();
                return token;
            }
        }
    }
}