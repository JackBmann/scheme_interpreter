
#[cfg(test)]

extern crate updated_scheme;

use updated_scheme::predicate::*;
use updated_scheme::evaluator::*;
use updated_scheme::interpreter::*;

#[test]
fn test_raw_predicate() {

    let expr_1 = Expression::Number(1);
    let expr_2 = Expression::Number(2);
    let expr_3 = Expression::Number(3);
    let expr_4 = Expression::Number(4);

    let predicate = Predicate {
        operator: '>',
        l_hand: expr_1,
        r_hand: expr_2,
        if_true: expr_3,
        if_false: expr_4,
    };
    
    let result = predicate.evaluate();
    let value = match result {
        Expression::Number(a) => a,
        _ => panic!(),
    };

    assert_eq!(value, 4);

}


#[test]
fn test_with_multiple_variable_string() {

    let env = String::from("(define (x (+ 3 2))) (define (y 6))");
    let expr = String::from("(if (> x y) (* x y) (+ x y))");
    let val = interpret_with_environment_string(expr, env);
    assert_eq!(val, 11);

}
