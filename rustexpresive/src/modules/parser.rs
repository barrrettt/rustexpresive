use crate::modules::tokenizer::Token;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Literal(bool),  // True/False
    Not(Box<Expr>),  // !
    And(Box<Expr>, Box<Expr>),  // &&
    Or(Box<Expr>, Box<Expr>),   // ||
    Equal(Box<Expr>, Box<Expr>), //   ==
    NotEqual(Box<Expr>, Box<Expr>), //  !=
}

pub struct Parser<'a> {
    tokens: &'a [Token],
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser { tokens, current: 0 }
    }

    // Función para avanzar al siguiente token
    fn advance(&mut self) -> Option<&Token> {
        self.current += 1;
        self.tokens.get(self.current - 1)
    }

    // Función para ver el token actual
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    // Función para analizar la expresión
    pub fn parse(&mut self) -> Option<Expr> {
        self.parse_or()
    }

    // Parse para OR
    fn parse_or(&mut self) -> Option<Expr> {
        let mut left = self.parse_and()?;

        while let Some(Token::Or) = self.peek() {
            self.advance(); // Consumiendo el token "||"
            let right = self.parse_and()?;
            left = Expr::Or(Box::new(left), Box::new(right));
        }

        Some(left)
    }

    // Parse para AND
    fn parse_and(&mut self) -> Option<Expr> {
        let mut left = self.parse_equal()?;

        while let Some(Token::And) = self.peek() {
            self.advance(); // Consumiendo el token "&&"
            let right = self.parse_equal()?;
            left = Expr::And(Box::new(left), Box::new(right));
        }

        Some(left)
    }

    // Parse para las expresiones de comparación (== y !=)
    fn parse_equal(&mut self) -> Option<Expr> {
        let mut left = self.parse_not()?;

        while let Some(token) = self.peek() {
            match token {
                Token::Equal => {
                    self.advance(); // Consumiendo el token "=="
                    let right = self.parse_not()?;
                    left = Expr::Equal(Box::new(left), Box::new(right));
                }
                Token::NotEqual => {
                    self.advance(); // Consumiendo el token "!="
                    let right = self.parse_not()?;
                    left = Expr::NotEqual(Box::new(left), Box::new(right));
                }
                _ => break,
            }
        }

        Some(left)
    }

    // Parse para NOT
    fn parse_not(&mut self) -> Option<Expr> {
        if let Some(Token::Not) = self.peek() {
            self.advance(); // Consumiendo el token "!"
            let expr = self.parse_atom()?;
            return Some(Expr::Not(Box::new(expr)));
        }

        self.parse_atom()
    }

    // Parse para las expresiones atómicas (valores literales y paréntesis)
    fn parse_atom(&mut self) -> Option<Expr> {
        match self.peek()? {
            Token::True => {
                self.advance();
                Some(Expr::Literal(true))
            }
            Token::False => {
                self.advance();
                Some(Expr::Literal(false))
            }
            Token::LParen => {
                self.advance(); // Consumiendo "("
                let expr = self.parse()?;
                if let Some(Token::RParen) = self.peek() {
                    self.advance(); // Consumiendo ")"
                    Some(expr)
                } else {
                    panic!("Error: No se encontró un paréntesis de cierre");
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::tokenizer::tokenize;

    #[test]
    fn test_parser_logics() {
        let tokens = tokenize("true && false || !true");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse().unwrap();

        let expected = Expr::Or(
            Box::new(Expr::And(
                Box::new(Expr::Literal(true)),
                Box::new(Expr::Literal(false)),
            )),
            Box::new(Expr::Not(Box::new(Expr::Literal(true)))),
        );

        assert_eq!(expr, expected);
    }

    #[test]
    fn test_parser_parentheses() {
        let tokens = tokenize("!(true && false)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse().unwrap();
        
        let expected = Expr::Not(Box::new(Expr::And( 
            Box::new(Expr::Literal(true)), 
            Box::new(Expr::Literal(false)), 
        )));
        
        assert_eq!(expr, expected);
    }
}
