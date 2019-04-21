use super::token::Token;

pub struct AST {
    token: Token,
    children: Vec<AST>
}

impl AST {
    pub fn new(token: Token, children: Vec<AST>) -> AST {
        AST {
            token,
            children,
        }
    }

    pub fn new_leaf(token: Token) -> AST {
        AST {
            token,
            children: Vec::new(),
        }
    }

    pub fn token(&self) -> &Token {
        &self.token
    }

    pub fn left(&self) -> &AST {
        &self.children[0]
    }

    pub fn right(&self) -> &AST {
        &self.children[1]
    }
}
