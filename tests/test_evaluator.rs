#[cfg(test)]

extern crate updated_scheme;
pub use updated_scheme::evaluator::*;

#[test]
fn test_interpret_tree() {
    let a = Expression::Number(3);
    let b = Expression::Number(4);
    let c = Expression::Number(5);
    let f = Expression::Number(6);
    let g = Expression::Mult(vec![c,f]);
    let d = Expression::Plus(vec![b,g]);

    let e = Expression::Plus(vec![a,d]);
    let result = evaluate(&e);
    assert_eq!(result.ok(), Some(37));
}

