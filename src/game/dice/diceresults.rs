use std::fmt;

#[derive(Debug)]
pub struct DiceResults {
    pub results: Vec<i64>,
    pub dice_count: i64,
    pub dice_sides: i64,
    pub addition: i64,
    pub sign: i64
}

impl DiceResults {
    pub fn get_result(&self) -> i64 {
        let res : i64 = self.results.iter().sum();
        return self.sign * self.addition + res;
    }
}

impl fmt::Display for DiceResults {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sign = "+";
        if self.sign < 0 {
            sign = "-";
        }
        println!("{} {}", sign, self.sign);
       return write!(f,"({count}d{sides}{sign}{addition} : {result})", count = self.dice_count, sides= self.dice_sides, addition= self.addition, result = self.get_result(), sign = sign);
    }
}
