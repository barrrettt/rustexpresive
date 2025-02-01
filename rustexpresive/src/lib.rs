mod modules;
use modules::tokenizer::tokenize;
use modules::parser::Parser;
use modules::interprete::evaluate;

pub fn execute(expresion: String) -> Result<bool, String> {
    let tokens = tokenize(&expresion);
    let mut parser = Parser::new(&tokens);
    match parser.parse() {
        Some(parsed_expr) => Ok(evaluate(parsed_expr)),
        None => Err("Error: Not valid expresion".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use crate::execute;


    #[test]
    fn test_complex_logic() {
        let input = "((true && false) || true) && !(false || true) && !(true && false)";
        assert_eq!(execute(input.to_string()).unwrap(), false);
    }
}
