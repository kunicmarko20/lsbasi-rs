#[derive(Debug, Copy, Clone)]
pub enum Token {
    INT(i64),
    OPERATION(Operation),
    NONE,
}

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    PLUS,
    MINUS,
}

impl Token {
    pub fn into_int(self) -> Result<i64, String> {
        if let Token::INT(value) = self {
            return Ok(value);
        }

        Err(String::from("Value not integer."))
    }
}
