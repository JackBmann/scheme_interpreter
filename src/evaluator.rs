use predicate::*;


#[derive(Clone)]
pub enum Expression {
    Number(f64),
    Plus(Vec<Expression>),
    Mult(Vec<Expression>),
    Predicate(Box<Predicate>),
}

pub fn evaluate(e: &Expression) -> Result<f64, &'static str> {
    match e {
        &Expression::Number(a)            => Ok(a),
        &Expression::Plus(ref v)          => Ok(v.iter().map(|y| evaluate(y).ok().unwrap()).fold(0.0, |acc, x| acc+x)),
        &Expression::Mult(ref v)          => Ok(v.iter().map(|y| evaluate(y).ok().unwrap()).fold(1.0, |acc, x| acc*x)),
        &Expression::Predicate(ref p)     => evaluate(&p.evaluate()),
    }
}
