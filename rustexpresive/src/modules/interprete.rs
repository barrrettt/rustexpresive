
use crate::modules::parser::Expr;

pub fn evaluate(expr: Expr) -> bool {
    match expr {
        // Literal booleano
        Expr::Literal(value) => value,
        
        // Negación lógica
        Expr::Not(boxed_expr) => !evaluate(*boxed_expr),
        
        // Operación AND
        Expr::And(boxed_left, boxed_right) => evaluate(*boxed_left) && evaluate(*boxed_right),
        
        // Operación OR
        Expr::Or(boxed_left, boxed_right) => evaluate(*boxed_left) || evaluate(*boxed_right),
        
        // Igualdad
        Expr::Equal(boxed_left, boxed_right) => evaluate(*boxed_left) == evaluate(*boxed_right),
        
        // Desigualdad
        Expr::NotEqual(boxed_left, boxed_right) => evaluate(*boxed_left) != evaluate(*boxed_right),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::tokenizer::tokenize;
    use crate::modules::parser::Parser;

    #[test]
    fn test_evaluate_logics() {
        let tokens = tokenize("true && false || !true");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse().unwrap();

        let result = evaluate(expr);
        assert_eq!(result, false); // El resultado de "true && false || !true" es false
    }

    #[test]
    fn test_evaluate_logics_parentheses() {
        let tokens = tokenize("!(true && false)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse().unwrap();

        let result = evaluate(expr);
        assert_eq!(result, true); // El resultado de "!(true && false)" es true
    }

    #[test]
    fn test_evaluate_logics_complex() {
        let tokens = tokenize("!(true!=false)&&(!(true))");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse().unwrap();
        let result = evaluate(expr);
        assert_eq!(result, false); 
    }

    #[test]
    #[should_panic(expected = "Error: No se encontró un paréntesis de cierre")]
    fn test_parser_unmatched_parentheses() {
        let tokens = tokenize("(true && false");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse().unwrap();
    
        // Este test debería hacer que el parser falle debido al paréntesis sin cerrar
        evaluate(expr); 
    }
}
