

use evaluator::*;
use parser::*;
use lexer::*;
use environment::*;
use environment_parser::*;

pub fn interpret_with_environment(s:String, e: Environment) -> i32 {

    // apply each element in environment to the string
    // this is *applicitave form*, not *normal form*.
    // Scheme is, by definition, applicative form.
    // Also note that this is not very smart... this should
    // be done by the parser, not with string replacement
    let mut s = s;
    for key in e.variables.keys() {
        s = s.replace(key,e.variables.get(key).unwrap());
    }
    interpret(s)

}

fn replace_from_environment(s:String, e:&Environment) -> String {
    for key in e.variables.keys() {
        if key == &s { return e.variables.get(key).unwrap().to_string(); }
    }
    return s;
}

pub fn interpret_with_environment_2(s:String, e: Environment) -> i32 {
    let original_tokens = tokenize(&s);
    let mut tokens = Vec::<Token>::new();
    for token in original_tokens {
        match token {
            Token::Constant(a) => tokens.push(Token::Constant(replace_from_environment(a,&e))),
            _ => tokens.push(token),
        }
    }
    let expression = parse(&mut tokens);
    let result = evaluate(&expression);
    return result.unwrap();
}

pub fn interpret_with_environment_string(s:String, e:String) -> i32 {
    let mut env_tokens = tokenize(&e);
    let env = parse_to_environment(&mut env_tokens);
    interpret_with_environment_2(s,env)
}

pub fn interpret(s: String) -> i32 {

    let mut tokens = tokenize(&s);
    let expression = parse(&mut tokens);
    let result = evaluate(&expression);
    return result.unwrap();

}
