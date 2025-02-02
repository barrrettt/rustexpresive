mod modules;
use modules::tokenizer::tokenize;
use modules::parser::Parser;
use modules::interprete::evaluate;

pub fn execute(expresion: String) -> Result<String, String> {
    let tokens = tokenize(&expresion);
    let mut parser = Parser::new(&tokens);
    match parser.parse() {
        Some(parsed_expr) =>{
            let result = evaluate(parsed_expr);
            match result {
                modules::interprete::EvalResult::Bool(b) => Ok(b.to_string()),
                modules::interprete::EvalResult::String(s) => Ok(s.to_string()),
            }
        },
        None => Err("Error: Not valid expresion".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use crate::execute;

    #[test]
    fn test_complex_logic() {
        let input = "((true && false) || true) && !(false || true) && !(true && false)".to_string();
        assert_eq!(execute(input).unwrap(), "false");
    }

    #[test]
    fn test_complex_strings_and_logics() {
        let input = r#"((true && "un coala \"pepe\"" != "texto") || true) && !("uno"!="dos" || true) && !("helloworld"=="camel" && false)"#.to_string();
        assert_eq!(execute(input).unwrap(), "false");
    }
}
