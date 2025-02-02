
use crate::modules::parser::Expr;
use std::fmt;

pub enum EvalResult {
    Bool(bool),
    String(String),
}

impl PartialEq for EvalResult {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (EvalResult::Bool(l), EvalResult::Bool(r)) => l == r,
            (EvalResult::String(l), EvalResult::String(r)) => l == r,
            _ => false,
        }
    }
}

impl fmt::Debug for EvalResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            EvalResult::Bool(ref b) => write!(f, "Bool({})", b),
            EvalResult::String(ref s) => write!(f, "Str({})", s),
        }
    }
}


pub fn evaluate(expr: Expr) -> EvalResult {
    match expr {
        // Literal booleano
        Expr::BooleanLiteral(value) => EvalResult::Bool(value),

        // String
        Expr::StringLiteral(s) => EvalResult::String(s),

        // Negación lógica
        Expr::Not(boxed_expr) => {
            let result = evaluate(*boxed_expr);
            match result {
                EvalResult::Bool(value) => EvalResult::Bool(!value),
                EvalResult::String(_) => panic!("No se puede aplicar NOT a un String"),
            }
        }

        // Operación AND
        Expr::And(boxed_left, boxed_right) => {
            let left = evaluate(*boxed_left);
            let right = evaluate(*boxed_right);

            match (left, right) {
                (EvalResult::Bool(l), EvalResult::Bool(r)) => EvalResult::Bool(l && r),
                _ => panic!("Ambos operandos deben ser booleanos para AND"),
            }
        }

        // Operación OR
        Expr::Or(boxed_left, boxed_right) => {
            let left = evaluate(*boxed_left);
            let right = evaluate(*boxed_right);

            match (left, right) {
                (EvalResult::Bool(l), EvalResult::Bool(r)) => EvalResult::Bool(l || r),
                _ => panic!("Ambos operandos deben ser booleanos para OR"),
            }
        }

        // Igualdad
        Expr::Equal(boxed_left, boxed_right) => {
            let left = evaluate(*boxed_left);
            let right = evaluate(*boxed_right);

            match (left, right) {
                (EvalResult::Bool(l), EvalResult::Bool(r)) => EvalResult::Bool(l == r),
                (EvalResult::String(l), EvalResult::String(r)) => EvalResult::Bool(l == r),
                _ => panic!("Solo puedes comparar booleanos o cadenas con igualdad"),
            }
        }

        // Desigualdad
        Expr::NotEqual(boxed_left, boxed_right) => {
            let left = evaluate(*boxed_left);
            let right = evaluate(*boxed_right);

            match (left, right) {
                (EvalResult::Bool(l), EvalResult::Bool(r)) => EvalResult::Bool(l != r),
                (EvalResult::String(l), EvalResult::String(r)) => EvalResult::Bool(l != r),
                _ => panic!("Solo puedes comparar booleanos o cadenas con desigualdad"),
            }
        }
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
        assert_eq!(result, EvalResult::Bool(false)); // El resultado de "true && false || !true" es false
    }

    #[test]
    fn test_evaluate_logics_parentheses() {
        let tokens = tokenize("!(true && false)");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse().unwrap();

        let result = evaluate(expr);
        assert_eq!(result, EvalResult::Bool(true)); // El resultado de "!(true && false)" es true
    }

    #[test]
    fn test_evaluate_logics_complex() {
        let tokens = tokenize("!(true!=false)&&(!(true))");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse().unwrap();
        
        let result = evaluate(expr);
        assert_eq!(result, EvalResult::Bool(false)); // El resultado de "!(true != false) && (!(true))" es false
    }

    #[test]
    fn test_evaluate_string_comparison_equal() {
        let tokens = tokenize("\"hola\" == \"hola\"");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse().unwrap();
        
        let result = evaluate(expr);
        assert_eq!(result, EvalResult::Bool(true)); // "hola" == "hola" es true
    }

    #[test]
    fn test_evaluate_string_comparison_not_equal() {
        let tokens = tokenize("\"hola\" != \"mundo\"");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse().unwrap();

        let result = evaluate(expr);
        assert_eq!(result, EvalResult::Bool(true)); // "hola" != "mundo" es true
    }

    #[test]
    fn test_evaluate_string_and_boolean_comparison() {
        let tokens = tokenize("\"hola\" == true");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse().unwrap();

        // Este caso debería hacer un pánico porque no se puede comparar un booleano con una cadena
        let result = std::panic::catch_unwind(|| evaluate(expr));
        assert!(result.is_err()); // Se espera un error
    }

    #[test]
    fn test_evaluate_boolean_and_string() {
        let tokens = tokenize("true == \"true\"");
        let mut parser = Parser::new(&tokens);
        let expr = parser.parse().unwrap();

        // Este caso también debería hacer un pánico porque no se puede comparar un booleano con una cadena
        let result = std::panic::catch_unwind(|| evaluate(expr));
        assert!(result.is_err()); // Se espera un error
    }
    
    #[test]
    #[should_panic(expected = "Error: No se encontró un paréntesis de cierre")]
    fn test_evaluate_invalid_parentheses() {
        let tokens = tokenize("(true && false");
        let mut parser = Parser::new(&tokens);
        parser.parse().unwrap();
    }
}

