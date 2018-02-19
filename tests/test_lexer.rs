#[cfg(test)]

extern crate updated_scheme;
pub use updated_scheme::lexer::tokenize;

#[test]
fn test_lexer() {
    let x = tokenize("(+ 33(+ 4 5))");
    assert_eq!(x.len(), 9);		
}
