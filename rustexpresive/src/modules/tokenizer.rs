
#[derive(Debug, PartialEq)]
pub enum Token {
    True,
    False,
    Not,      // "!"
    NotEqual, // "!="
    Equal,    // "=="
    LParen,   // "("
    RParen,   // ")"
    And,      // "&&"
    Or,       // "||"
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
            ' ' => { chars.next(); } // Ignorar espacios 
            _ => panic!("Error: Caracter inesperado '{}'", ch), 
        }
    }

    tokens
}

#[test]
fn test_tokenize_logics() {
    assert_eq!(tokenize("true"), vec![Token::True]);
    assert_eq!(tokenize("false"), vec![Token::False]);
    assert_eq!(tokenize("!true"), vec![Token::Not, Token::True]);
    assert_eq!(tokenize("true == false"), vec![Token::True, Token::Equal, Token::False]);
    assert_eq!(tokenize("!(true != false)"), vec![
        Token::Not, Token::LParen, Token::True, Token::NotEqual, Token::False, Token::RParen
    ]);
    assert_eq!(tokenize("!(false && true)"), vec![
        Token::Not, Token::LParen, Token::False, Token::And, Token::True, Token::RParen
    ]);
    assert_eq!(tokenize("true || false"), vec![
        Token::True, Token::Or, Token::False
    ]);
    
    let result = tokenize("!(true!=false)&&(!(true))");
    println!("{:?}", result);
    assert_eq!(result, vec![
        Token::Not, Token::LParen, Token::True, Token::NotEqual, Token::False, Token::RParen,Token::And, Token::LParen, Token::Not, Token::LParen, Token::True, Token::RParen, Token::RParen]);
}
