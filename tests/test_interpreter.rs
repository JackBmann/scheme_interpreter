#[cfg(test)]

extern crate updated_scheme;
pub use updated_scheme::interpreter::*;

#[test]
fn test_single_number() {
    let val = interpret(String::from("88"));
    assert_eq!(val,88);
}

#[test]
fn test_negative_number() {
    let val = interpret(String::from("-10"));
    assert_eq!(val,-10);
}

#[test]
fn test_simple_evaluation() {
    let val = interpret(String::from("(+ 2 2)"));
    assert_eq!(val, 4);
}

#[test]
fn test_multiple_parens() {
    let val = interpret(String::from("(* (+ 7 6) (+ 2 3) (* 9 9))"));
    assert_eq!(val, 0);
}
