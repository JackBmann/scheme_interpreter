use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

extern crate updated_scheme;

use updated_scheme::lexer::*;
use updated_scheme::parser::*;
use updated_scheme::evaluator::*;
use updated_scheme::environment::*;
use updated_scheme::environment_parser::*;

fn parse_and_evaluate(tokens: &mut Vec<Token>, env: &Environment) {
    let expression = parse(tokens, &env);
    let result = evaluate(&expression);
    println!("{}", result.ok().unwrap());
}

fn parse_and_add_to_environment(tokens: &mut Vec<Token>, env: &mut Environment) {
    let new_env = parse_to_environment(tokens);
    env.variables = env.variables.clone().into_iter().chain(new_env.variables).collect();
}

fn main() {

    let mut variables: HashMap<String,Expression> = HashMap::new();
    let mut environment = Environment { variables: variables };

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();

        let mut tokens = tokenize(&line);
        let second_token = tokens.get(1).unwrap().clone();

        match second_token {
            Token::Keyword(ref k) => {
                match k {
                    &Keyword::Define => parse_and_add_to_environment(&mut tokens, &mut environment),
                    _ => parse_and_evaluate(&mut tokens, &environment),
                }
            }
            _ => parse_and_evaluate(&mut tokens, &environment),
        };
    }
}
