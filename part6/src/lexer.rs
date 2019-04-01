use super::token::Token;

pub struct Lexer {
    text: String,
    position: usize,
    current_character: Option<char>,
}

impl Lexer {
    pub fn new<T: Into<String>>(text: T) -> Self {
        let text_as_string = text.into();
        Lexer{
            current_character: (&text_as_string).chars().next(),
            text: text_as_string,
            position: 0,
        }
    }

    pub fn advance(&mut self) {
        self.position += 1;
        self.current_character = self.text.chars().nth(self.position);
    }

    fn skip_whitespace(&mut self) {
        while let Some(value) = self.current_character {
            if value.is_whitespace() {
                self.advance();
                continue;
            }
            return;
        }
    }

    fn integer(&mut self) -> u32 {
        let mut result = String::new();

        while let Some(character) = self.current_character {
            if character.is_digit(10) {
                result += &character.to_string();
                self.advance();
                continue;
            }
            break;
        }

        result.parse::<u32>().unwrap()
    }

    pub fn get_next_token(&mut self) -> Token {
        while let Some(character) = self.current_character {
            if character.is_whitespace() {
                self.skip_whitespace();
                continue;
            }


            if character.is_digit(10) {
                return Token::Integer(self.integer());
            }

            match character {
                '+' => {
                    self.advance();
                    return Token::Plus;
                },
                '-' => {
                    self.advance();
                    return Token::Minus;
                },
                '*' => {
                    self.advance();
                    return Token::Mul;
                },
                '/' => {
                    self.advance();
                    return Token::Div;
                },
                '(' => {
                    self.advance();
                    return Token::LParen;
                },
                ')' => {
                    self.advance();
                    return Token::RParen;
                },
                _ => ()
            }
        }

        return Token::End;
    }
}