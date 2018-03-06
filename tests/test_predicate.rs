
#[cfg(test)]

extern crate updated_scheme;

use updated_scheme::lexer::Token;
use updated_scheme::predicate::*;
use updated_scheme::evaluator::*;
use updated_scheme::interpreter::*;

#[test]
fn test_raw_predicate() {

    let expr_1 = Expression::Number(1.0);
    let expr_2 = Expression::Number(2.0);
    let expr_3 = Expression::Number(3.0);
    let expr_4 = Expression::Number(4.0);

    let predicate = Predicate {
        operator: Token::Oper('>'),
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

    assert_eq!(value, 4.0);

}


#[test]
fn test_with_multiple_variable_string_greater_than_true() {

    let env = String::from("(define (x (+ 3 4))) (define (y 6))");
    let expr = String::from("(if (> x y) (* x y) (+ x y))");
    let val = interpret_with_environment_string(expr, env);
    assert_eq!(val, 42.0);

}

#[test]
fn test_with_multiple_variable_string_greater_than_false() {

    let env = String::from("(define (x (+ 3 3))) (define (y 6))");
    let expr = String::from("(if (> x y) (* x y) (+ x y))");
    let val = interpret_with_environment_string(expr, env);
    assert_eq!(val, 12.0);

}

#[test]
fn test_with_multiple_variable_string_less_than_true() {

    let env = String::from("(define (x (+ 3 2))) (define (y 6))");
    let expr = String::from("(if (< x y) (* x y) (+ x y))");
    let val = interpret_with_environment_string(expr, env);
    assert_eq!(val, 30.0);

}

#[test]
fn test_with_multiple_variable_string_less_than_false() {

    let env = String::from("(define (x (+ 3 3))) (define (y 6))");
    let expr = String::from("(if (< x y) (* x y) (+ x y))");
    let val = interpret_with_environment_string(expr, env);
    assert_eq!(val, 12.0);

}

#[test]
fn test_with_multiple_variable_string_equal_true() {

    let env = String::from("(define (x (+ 3 2))) (define (y 5))");
    let expr = String::from("(if (= x y) (* x y) (+ x y))");
    let val = interpret_with_environment_string(expr, env);
    assert_eq!(val, 25.0);

}

#[test]
fn test_with_multiple_variable_string_equal_false() {

    let env = String::from("(define (x (+ 3 2))) (define (y 6))");
    let expr = String::from("(if (= x y) (* x y) (+ x y))");
    let val = interpret_with_environment_string(expr, env);
    assert_eq!(val, 11.0);

}
