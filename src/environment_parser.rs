
use lexer::*;
use parser::*;
use environment::*;
use evaluator::*;
use std::collections::HashMap;

pub fn parse_to_environment(mut v: &mut Vec<Token>) -> Environment {

	// a hash to store our environments
    let mut hash: HashMap<String,Expression> = HashMap::new();
    let mut environment = Environment { variables: hash };
	
	// until we have parsed all environment elements
	while v.len() > 0 {
	
		// all environments start with a lparen
		// environments are non-recursive
		let lparen = v.remove(0);
		match lparen { Token::LParen => (), _ => panic!() }

		// next word must be a "define"
		match v.remove(0) {
            Token::Keyword(k) => match k { Keyword::Define => {}, _ => panic!() },
		    _ => panic!(),
		}

		// next must be another lparen
		match v.remove(0) { Token::LParen => (), _ => panic!() }

		// next will be a symbol and the name
		let name_token = v.remove(0);
		//let value_token = v.remove(0);

		let name = match name_token { Token::Constant(s) => s, _ => panic!() };
		//let value = match value_token { Token::Constant(s) => s, _ => panic!() };

        let value = parse(v,&environment);

		// lastly, two rparens
		match v.remove(0) { Token::RParen => (), _ => panic!() }
		match v.remove(0) { Token::RParen => (), _ => panic!() }

		environment.variables.insert(name, value);
 	}
	
	// create the environment and return
    environment
}
