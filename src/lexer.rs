
pub enum Token {
    LParen,
    RParen,
    Oper(char),
    Constant(Vec<char>),
}

enum TokenizationState {
    Starting,
    Accumulating,
}

fn is_single_character_token(c: char) -> bool {
    match c {
        '('   => true,
        ')'   => true,
        '+'   => true,
        '*'   => true,
        _     => false,
    }
}

fn get_single_character_token(c: char) -> Token {
    match c {
        '('   => Token::LParen,
        ')'   => Token::RParen,
        _     => Token::Oper(c),
    }
}

fn is_whitespace(c: char) -> bool {
    c.is_whitespace()
}

pub fn tokenize(s: &str) -> Vec<Token> {
    
    let mut state: TokenizationState = TokenizationState::Starting;
    let mut tokens: Vec<Token> = Vec::<Token>::new();
    let mut accumulation: Vec<char> = Vec::<char>::new();

    for c in s.chars() {
        match state {
            TokenizationState::Starting => {
                if is_single_character_token(c) {
                    tokens.push(get_single_character_token(c));
                } else if is_whitespace(c) {
                    // ignore
                } else {
                    accumulation.push(c);
                    state = TokenizationState::Accumulating;
                }
            },
            TokenizationState::Accumulating => {
                if is_single_character_token(c) {
                    state = TokenizationState::Starting;
                    tokens.push(Token::Constant(accumulation.clone()));
                    accumulation.clear();
                    tokens.push(get_single_character_token(c));
                } else if is_whitespace(c) {
                    state = TokenizationState::Starting;
                    tokens.push(Token::Constant(accumulation.clone()));
                    accumulation.clear();
                } else {
                    accumulation.push(c);
                }
            },
        }
    }

    return tokens;
}
