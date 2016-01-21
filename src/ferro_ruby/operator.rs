use std::fmt;

#[derive(Clone)]
pub struct Operator {
    pub symbol: String
}

impl Operator {
    pub fn call(&self, operand: i64, summand: i64) -> Result<i64, &str> {
        if &self.symbol == "+" {
            let result = operand + summand;
            return Ok(result);
        }
        if &self.symbol == "-" {
            let result = operand - summand;
            return Ok(result);
        }
        return Err("Invalid Operator Error");
    }
}

impl fmt::Debug for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.symbol)
    }
}
