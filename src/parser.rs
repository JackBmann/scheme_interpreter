
use lexer::Token;
use evaluator::Expression;


fn parse_compound(mut tokens: &mut Vec<Token>) -> Expression {

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
            _                  => expressions.push(parse(tokens)),
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

pub fn parse(mut tokens: &mut Vec<Token>) -> Expression {
    // if integer, return integer
    let tok = tokens.remove(0);
    match tok {
        Token::Constant(s) => Expression::Number(s.parse::<i32>().unwrap()),
        Token::LParen      => parse_compound(tokens),
        _ => panic!(),
    }

}

