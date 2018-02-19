pub enum Expression {
    Number(i32),
    Plus(Vec<Expression>),
    Mult(Vec<Expression>),
}

pub fn interpret(e: &Expression) -> Result<i32, &'static str> {
    match e {
        &Expression::Number(a)            => Ok(a),
        &Expression::Plus(ref v)          => Ok(v.iter().map(|y| interpret(y).ok().unwrap()).fold(0, |acc, x| acc+x)),
        &Expression::Mult(ref v)          => Ok(v.iter().map(|y| interpret(y).ok().unwrap()).fold(1, |acc, x| acc*x)),

    }
}


