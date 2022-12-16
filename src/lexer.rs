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
    Period,
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
            panic!("Invalid value for curr_pos {}", self.curr_pos);
        }
        return self.current_line.chars().nth(self.curr_pos).unwrap();
    }

    pub fn march_pos(&mut self) -> char {
        if self.curr_pos < self.line_size {
            self.curr_pos += 1;
        }
        if self.curr_pos == self.line_size {
            return ' ';
        }
        return self.current_line.chars().nth(self.curr_pos).unwrap();
    }

    pub fn unmarch_pos(&mut self) -> char {
        if self.curr_pos > 0 {
            self.curr_pos -= 1;
        } else {
            warn!("Invalid attempt to move before the first character of input");
            return self.current_line.chars().nth(self.curr_pos).unwrap();
        }
        return self.current_line.chars().nth(self.curr_pos).unwrap();
    }

    pub fn get_next_token(&mut self) -> () {
        if self.curr_pos == self.line_size {
            debug!("No more characters!");
            self.curr_tok.token_type = TokType::End;
            return ();
        }
        let mut ch = self.get_curr_char();

        while ch == ' ' {
            ch = self.march_pos();
        }

        match ch {
            '\n' => self.curr_tok.token_type = TokType::Newl,
            '*' => self.curr_tok.token_type = TokType::Mul,
            '/' => self.curr_tok.token_type = TokType::Div,
            '+' => self.curr_tok.token_type = TokType::Plus,
            '-' => self.curr_tok.token_type = TokType::Minus,
            '(' => self.curr_tok.token_type = TokType::Lpar,
            ')' => self.curr_tok.token_type = TokType::Rpar,
            '=' => self.curr_tok.token_type = TokType::Equal,
            'x' => {
                self.curr_tok.token_type = TokType::Xvar;
                self.curr_tok.token_content = String::from("x");
            }
            'y' => {
                self.curr_tok.token_type = TokType::Xvar;
                self.curr_tok.token_content = String::from("y");
            }
            'z' => {
                self.curr_tok.token_type = TokType::Xvar;
                self.curr_tok.token_content = String::from("z");
            }
            '^' => self.curr_tok.token_type = TokType::Caret,
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                let mut number = 0.0;
                while ch.is_digit(10) {
                    let get_digit = ch.to_digit(10);
                    let mut digit: f32 = 0.0;
                    match get_digit {
                        Some(d) => digit = d as f32,
                        None => {
                            error!("Failed to parse {} as digit", ch);
                        }
                    }
                    number = 10.0 * number + digit;
                    ch = self.march_pos();
                }
                if ch == '.' {
                    ch = self.march_pos();
                    let mut exp = -1;
                    while ch.is_digit(10) {
                        let get_digit = ch.to_digit(10);
                        let mut digit: f32 = 0.0;
                        match get_digit {
                            Some(d) => digit = d as f32,
                            None => {
                                error!("Failed to parse {} as digit", ch);
                            }
                        }
                        number = number + digit * f32::powi(10.0, exp);
                        exp -= 1;
                        ch = self.march_pos();
                    }
                }
                ch = self.unmarch_pos();
                self.curr_tok.token_type = TokType::Number;
                self.curr_tok.token_content = format!("{number}");
            }
            '.' => self.curr_tok.token_type = TokType::Period,
            _ => {
                error!("Unknown character: {} ", ch);
                ()
            }
        }
        info!(
            "Found token {:#?} with {}, {}",
            self.curr_tok.token_type, ch, self.curr_tok.token_content
        );

        self.march_pos();
    }
}

pub fn tokenize(mut lexer: Lexer) {
    let mut ind = 0;
    lexer.get_next_token();
    loop {
        if lexer.curr_tok.token_type == TokType::Newl {
            break;
        }
        if lexer.curr_tok.token_type == TokType::End {
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
        assert_eq!(lexer.curr_pos, 1);
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
        assert_eq!(lexer.curr_pos, 10);
    }

    #[test]
    fn test_lexer_numbers() {
        let string = String::from("2.3\n");
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
        let token = lexer.curr_tok;
        assert_eq!(token.token_type, TokType::Number);
        assert_eq!(token.token_content, String::from("2.3"));
    }
}
