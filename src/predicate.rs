
use evaluator::Expression;
use evaluator::evaluate;

#[derive(Clone)]
pub struct Predicate {
    pub operator: char,
    pub l_hand: Expression,
    pub r_hand: Expression,
    pub if_true: Expression,
    pub if_false: Expression,
}

impl Predicate {

    pub fn evaluate(&self) -> Expression {
        let l_val = evaluate(&self.l_hand).ok().unwrap();
        let r_val = evaluate(&self.r_hand).ok().unwrap();
        match self.operator {
            '>' => if l_val > r_val  { return self.if_true.clone() } else { return self.if_false.clone() },
            _ => panic!("unknown operator in predicate")
        }
        return self.if_true.clone();
    }

}
