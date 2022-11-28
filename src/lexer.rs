use std::io;

#[derive(PartialEq, Debug)]
pub enum TokType {
    Exit,
    Newl,
    Mul,
    Div,
    Plus,
    PlusEq,
    Minus,
    MinusEq,
    Lpar,
    Rpar,
    Equal,
    Number,
    Xvar,
    Caret,
    Identifier,
    UnknownToken,
    End,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokType,
    pub token_content: String,
}

pub struct Lexer {
    pub current_line: String,
    pub line_size: usize,
    pub curr_pos: usize,
    pub curr_tok: Token,
}

impl Lexer {
    fn lexer_init(current_line: String) -> Self {
        Lexer {
            current_line: current_line.clone(),
            line_size: current_line.len(),
            curr_pos: 0,
            curr_tok: Token {
                token_type: TokType::End,
                token_content: String::from(""),
            },
        }
    }

    pub fn get_curr_char(&self) -> char {
        if self.curr_pos > self.line_size {
            println!("Invalid value for curr_pos {}", self.curr_pos);
        }
        return self.current_line.chars().nth(self.curr_pos).unwrap();
    }

    pub fn march_pos(&mut self) -> char {
        if self.curr_pos < self.line_size - 1 {
            self.curr_pos += 1;
        } else {
            println!("Invalid attempt to move after the last character of input");
        }
        return self.current_line.chars().nth(self.curr_pos).unwrap();
    }

    pub fn unmarch_pos(&mut self) -> char {
        if self.curr_pos > 0 {
            self.curr_pos -= 1;
        } else {
            println!("Invalid attempt to move before the first character of input");
        }
        return self.current_line.chars().nth(self.curr_pos).unwrap();
    }

    pub fn get_next_token(&mut self) -> () {
        let mut ch = self.get_curr_char();

        while ch == ' ' {
            ch = self.march_pos();
        }

        match ch {
            '\n' => {
                self.curr_tok.token_type = TokType::Newl;
                return ();
            }
            '*' => self.curr_tok.token_type = TokType::Mul,
            '/' => self.curr_tok.token_type = TokType::Div,
            '+' => self.curr_tok.token_type = TokType::Plus,
            '-' => self.curr_tok.token_type = TokType::Minus,
            '(' => self.curr_tok.token_type = TokType::Lpar,
            ')' => self.curr_tok.token_type = TokType::Rpar,
            '=' => self.curr_tok.token_type = TokType::Equal,
            'x' | 'y' | 'z' => self.curr_tok.token_type = TokType::Xvar,
            '^' => self.curr_tok.token_type = TokType::Caret,
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '.' => {
                self.curr_tok.token_type = TokType::Number
            }
            _ => {
                println!("Unknown character: {} ", ch);
                ()
            }
        }
        println!("Found token {:#?} with {}", self.curr_tok.token_type, ch);

        self.march_pos();
    }
}

pub fn tokenize(mut lexer: Lexer) {
    let mut ind = 0;
    lexer.get_next_token();
    while lexer.curr_tok.token_type != TokType::Newl {
        if lexer.curr_pos == lexer.line_size - 1 {
            break;
        }
        lexer.get_next_token();
        ind += 1;
        if ind > 100 {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[test]
    fn test_get_next_token_one_char() {
        let string = String::from("x");
        let mut lexer = Lexer {
            current_line: string.clone(),
            line_size: string.len(),
            curr_pos: 0,
            curr_tok: Token {
                token_type: TokType::End,
                token_content: String::from(""),
            },
        };
        lexer.get_next_token();
        assert_eq!(lexer.curr_tok.token_type, TokType::Xvar);
        assert_eq!(lexer.curr_pos, 0);
    }

    #[test]
    fn test_get_next_token_initial() {
        let string = String::from("x + y + z\n");
        let mut lexer = Lexer {
            current_line: string.clone(),
            line_size: string.len(),
            curr_pos: 0,
            curr_tok: Token {
                token_type: TokType::End,
                token_content: String::from(""),
            },
        };
        lexer.get_next_token();
        assert_eq!(lexer.curr_tok.token_type, TokType::Xvar);
        assert_eq!(lexer.curr_pos, 1);
        lexer.get_next_token();
        assert_eq!(lexer.curr_tok.token_type, TokType::Plus);
        assert_eq!(lexer.curr_pos, 3);
        lexer.get_next_token();
        assert_eq!(lexer.curr_tok.token_type, TokType::Xvar);
        assert_eq!(lexer.curr_pos, 5);
        lexer.get_next_token();
        assert_eq!(lexer.curr_tok.token_type, TokType::Plus);
        assert_eq!(lexer.curr_pos, 7);
        lexer.get_next_token();
        assert_eq!(lexer.curr_tok.token_type, TokType::Xvar);
        assert_eq!(lexer.curr_pos, 9);
        lexer.get_next_token();
        assert_eq!(lexer.curr_tok.token_type, TokType::Newl);
        assert_eq!(lexer.curr_pos, 9);
    }
}
