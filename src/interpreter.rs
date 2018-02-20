

use evaluator::*;
use parser::*;
use lexer::*;

pub fn interpret(s: String) -> i32 {

    let mut tokens = tokenize(&s);
    let expression = parse(&mut tokens);
    let result = evaluate(&expression);
    return result.unwrap();

}
