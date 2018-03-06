

use evaluator::*;
use parser::*;
use lexer::*;
use environment::*;
use environment_parser::*;
use std::collections::HashMap;

pub fn interpret_with_environment(s:String, e: Environment) -> f64 {
    let mut tokens = tokenize(&s);
    let expression = parse(&mut tokens, &e);
    let result = evaluate(&expression);
    return result.unwrap();
}

pub fn interpret_with_environment_string(s:String, e:String) -> f64 {
    let mut env_tokens = tokenize(&e);
    let env = parse_to_environment(&mut env_tokens);
    interpret_with_environment(s,env)
}

pub fn interpret(s: String) -> f64 {
    let hash: HashMap<String, Expression> = HashMap::new();
    let env = Environment { variables: hash };
    let mut tokens = tokenize(&s);
    let expression = parse(&mut tokens, &env);
    let result = evaluate(&expression);
    return result.unwrap();

}
