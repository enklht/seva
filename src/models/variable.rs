use super::Value;

#[derive(Clone)]
pub enum Variable {
    External(Value),
    Internal(Value),
}

impl Variable {
    pub fn get(&self) -> Value {
        use Variable::*;
        match self {
            External(n) => n.clone(),
            Internal(n) => n.clone(),
        }
    }

    pub fn is_external(&self) -> bool {
        matches!(self, Variable::External(_))
    }
}
