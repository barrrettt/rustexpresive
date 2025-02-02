use std::arch::x86_64;


#[derive(Debug, PartialEq)]
pub enum Token {
    True,
    False,
    StringLiteral(String), // "Textos para comparar"
    Not,      // "!"
    NotEqual, // "!="
    Equal,    // "=="
    LParen,   // "("
    RParen,   // ")"
    And,      // "&&"
    Or,       // "||"
    Number(f64),  // Para números, tanto enteros como decimales positivos o negativos
    Plus,     // "+"
    Minus,    // "-"
    Multiply, // "*"
    Divide,   // "/"
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut chars = input.chars().peekable();
    let mut tokens = Vec::new();

    while let Some(&ch) = chars.peek() {
        match ch {
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            '!' => {
                chars.next();  
                if let Some(&'=') = chars.peek() {
                    chars.next();  
                    tokens.push(Token::NotEqual);
                } else {
                    tokens.push(Token::Not);
                }
            }
            '=' => {
                chars.next();  
                if let Some(&'=') = chars.peek() {
                    chars.next();  
                    tokens.push(Token::Equal);
                } else {
                    panic!("Error: '=' inesperado");
                }
            }
            '&' => {
                chars.next();  
                if let Some('&') = chars.peek() {
                    chars.next(); 
                    tokens.push(Token::And); 
                } else {
                    panic!("Error: Se esperaba otro '&'");
                }
            }
            '|' => {
                chars.next();  
                if let Some('|') = chars.peek() {
                    chars.next(); 
                    tokens.push(Token::Or); 
                } else {
                    panic!("Error: Se esperaba otro '|'");
                }
            }
            't' => {
                if chars.clone().take(4).collect::<String>() == "true" {
                    for _ in 0..4 { chars.next(); }  
                    tokens.push(Token::True);
                } else {
                    panic!("Error: Token inesperado");
                }
            }
            'f' => {
                if chars.clone().take(5).collect::<String>() == "false" {
                    for _ in 0..5 { chars.next(); }  
                    tokens.push(Token::False); 
                } else {
                    panic!("Error: Token inesperado");
                }
            }
            '"' => {
                // Manejar las cadenas entre comillas, incluyendo comillas escapadas
                let mut string_literal = String::new();
                chars.next(); // Consumimos la primera comilla
                let mut end = false;
                while let Some(&ch) = chars.peek() {
                    if ch == '"' {
                        chars.next();
                        end = true;
                        break;
                    } else if ch == '\\' {
                        // Si encontramos una barra invertida, verificamos si la siguiente es una comilla
                        chars.next(); 
                        if let Some(&next_ch) = chars.peek() {
                            if next_ch == '"' {
                                // Si la siguiente es una comilla, la agregamos a la cadena
                                string_literal.push('"');
                                chars.next(); // Consumimos la comilla
                            } else {
                                // Si no es una comilla, agregamos la barra invertida como parte de la cadena
                                string_literal.push('\\');
                                string_literal.push(next_ch);
                                chars.next();
                            }
                        }
                    } else {
                        // Cualquier otro carácter dentro de la cadena
                        string_literal.push(ch);
                        chars.next();
                    }
                }

                // Si no encontramos una comilla de cierre
                if !end {
                    panic!("Error: Comilla de cierre no encontrada para la cadena");
                }
                
                tokens.push(Token::StringLiteral(string_literal));
            }
            // Manejar números
            ch if ch.is_digit(10) || ch == '.' => {
                // Manejo de números (negativos en el parser)
                let mut number_str = String::new();

                // Capturamos la parte entera del número
                while let Some(&ch) = chars.peek() {
                    if ch.is_digit(10) {
                        number_str.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }

                // Capturamos el punto decimal si existe
                if let Some(&ch) = chars.peek() {
                    if ch == '.' {
                        number_str.push(ch);
                        chars.next();

                        // Capturamos los dígitos después del punto
                        while let Some(&ch) = chars.peek() {
                            if ch.is_digit(10) {
                                number_str.push(ch);
                                chars.next();
                            } else {
                                break;
                            }
                        }
                    }
                }
                // Convertimos el string a número (f64)
                let number: f64 = number_str.parse().unwrap();
                tokens.push(Token::Number(number));
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Multiply);
                chars.next();
            }
            '/' => {
                tokens.push(Token::Divide);
                chars.next();
            }

            // /////// Ignorar espacios en blanco
            ' ' => { chars.next(); } 
            _ => panic!("Error: Caracter inesperado '{}'", ch), 
        }
    }

    tokens
}


///////////////
/// TEST ⚠️///
///////////////

#[cfg(test)]
mod tests {
    use super::*; 

    //BASICS
    #[test]
    fn test_tokenize_true() {
        assert_eq!(tokenize("true"), vec![Token::True]);
    }

    #[test]
    fn test_tokenize_false() {
        assert_eq!(tokenize("false"), vec![Token::False]);
    }

    #[test]
    fn test_tokenize_not_true() {
        assert_eq!(tokenize("!true"), vec![Token::Not, Token::True]);
    }

    #[test]
    fn test_tokenize_true_equal_false() {
        assert_eq!(tokenize("true == false"), vec![Token::True, Token::Equal, Token::False]);
    }

    #[test]
    fn test_tokenize_not_paren_true_not_equal_false() {
        assert_eq!(tokenize("!(true != false)"), vec![
            Token::Not, Token::LParen, Token::True, Token::NotEqual, Token::False, Token::RParen
        ]);
    }

    #[test]
    fn test_tokenize_not_paren_false_and_true() {
        assert_eq!(tokenize("!(false && true)"), vec![
            Token::Not, Token::LParen, Token::False, Token::And, Token::True, Token::RParen
        ]);
    }

    #[test]
    fn test_tokenize_true_or_false() {
        assert_eq!(tokenize("true || false"), vec![
            Token::True, Token::Or, Token::False
        ]);
    }

    #[test]
    fn test_tokenize_complex_expression() {
        let result = tokenize("!(true!=false)&&(!(true))");
        println!("{:?}", result);
        assert_eq!(result, vec![
            Token::Not, Token::LParen, Token::True, Token::NotEqual, Token::False, Token::RParen,
            Token::And, Token::LParen, Token::Not, Token::LParen, Token::True, Token::RParen, Token::RParen
        ]);
    }

    // STRINGS
    #[test]
    fn test_tokenize_string_with_comparison() {
        assert_eq!(tokenize(r#" "Hola amigo"=="esto es una prueba" "#), vec![
            Token::StringLiteral(("Hola amigo").to_string()), 
            Token::Equal, 
            Token::StringLiteral(("esto es una prueba").to_string())
        ]);
    }

    #[test]
    fn test_tokenize_string_with_escape() {
        assert_eq!(tokenize(r#" "Hola \"amigo\"" != "esto es una \"prueba\"" "#), vec![
            Token::StringLiteral(("Hola \"amigo\"").to_string()),
            Token::NotEqual,
            Token::StringLiteral(("esto es una \"prueba\"").to_string())
        ]);
    }

    #[test] 
    fn test_tokenize_unmatched_quotes() {
        let result = std::panic::catch_unwind(|| {
            tokenize(r#""Hola mundo"#); // Comillas no cerradas 
        });
        assert!(result.is_err()); // Esperamos que se lance un pánico 
    } 
    // NUMEROS
    #[test]
    fn test_tokenize_numbers() {
        assert_eq!(tokenize("123"), vec![Token::Number(123.0)]);
        assert_eq!(tokenize("-123"), vec![Token::Minus, Token::Number(123.0)]);
        assert_eq!(tokenize("123.45"), vec![Token::Number(123.45)]);
        assert_eq!(tokenize("0.1"), vec![Token::Number(0.1)]);
        assert_eq!(tokenize(".5"), vec![Token::Number(0.5)]);
    }

    #[test]
    fn test_tokenize_operations() {
        assert_eq!(tokenize("-1 + 2"), vec![Token::Minus, Token::Number(1.0), Token::Plus, Token::Number(2.0)]);
        assert_eq!(tokenize("3 - 4"), vec![Token::Number(3.0), Token::Minus, Token::Number(4.0)]);
        assert_eq!(tokenize("5 * 6"), vec![Token::Number(5.0), Token::Multiply, Token::Number(6.0)]);
        assert_eq!(tokenize("7 / 8"), vec![Token::Number(7.0), Token::Divide, Token::Number(8.0)]);
    }

    #[test]
    fn test_tokenize_number_complex_expression() {
        let result = tokenize("1 + 2 * 3 - 4 / 5");
        assert_eq!(result, vec![
            Token::Number(1.0), Token::Plus, 
            Token::Number(2.0), Token::Multiply, Token::Number(3.0), 
            Token::Minus, Token::Number(4.0), Token::Divide, Token::Number(5.0)
        ]);
    }

}
