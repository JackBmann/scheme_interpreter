
use lexer::Token;
use evaluator::Expression;
use environment::Environment;

fn parse_compound(mut tokens: &mut Vec<Token>, environment: &Environment) -> Expression {

    // the ONLY possibility is for the first token to be an operator
    let operator = tokens.remove(0);
    let c = match operator {
        Token::Oper(o) => o,
        _              => panic!(),
    };

    // get vector of following expressions
    let mut expressions = Vec::<Expression>::new();
    loop {
        match tokens.get(0).unwrap() {
            &Token::RParen     => break,
            _                  => expressions.push(parse(tokens, environment)),
        }
    }
    // after breaking, remove last element
    tokens.remove(0);

    // create a token, must be + or *
    match c {
        '+'  => Expression::Plus(expressions),
        '*'  => Expression::Mult(expressions),
        _    => panic!(),
    }

}

fn constant_to_expression(s: String, environment: &Environment) -> Expression {
    for key in environment.variables.keys() {
        if &s == key {
            let expr = environment.variables.get(key).unwrap();
            return expr.clone();
        }
    }
    return Expression::Number(s.parse::<i32>().unwrap());
}

pub fn parse(mut tokens: &mut Vec<Token>, environment: &Environment) -> Expression {
    // if integer, return integer
    let tok = tokens.remove(0);
    match tok {
        Token::Constant(s) => constant_to_expression(s, environment),
        Token::LParen      => parse_compound(tokens, environment),
        _ => panic!(),
    }

}

