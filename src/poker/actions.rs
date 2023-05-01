pub const DUMMY_AMOUNTS: [i32; 6] = [10, 100, 500, 1000, 5000, 10000];

pub trait Action {}

#[derive(Debug)]
pub struct Call;
impl Action for Call {}
impl std::fmt::Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "call")
    }
}

#[derive(Debug)]
pub struct Fold;
impl Action for Fold {}
impl std::fmt::Display for Fold {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "fold")
    }
}

#[derive(Debug)]
pub struct Raise {
    amount:i32
}
impl Action for Raise {}
impl std::fmt::Display for Raise {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "raise")
    }
}
impl Raise {
    pub fn new() -> Self {
        Raise{
            amount: 0
        }
    }

    pub fn set_amount(&mut self, amount: i32) {
        self.amount = amount;
    }
}

#[derive(Debug)]
struct AbstractedRaise {
    amounts: Vec<i32>,
    amount: i32,
}
impl Action for AbstractedRaise {}
impl AbstractedRaise {
    pub fn new(allowed_amounts: Vec<i32>) -> Self {
        AbstractedRaise {
            amounts: allowed_amounts,
            amount: 0,
        }
    }

    fn call(&mut self, amount: i32) -> Result<(), String> {
        if !self.amounts.contains(&amount) {
            Err(format!(
                "Specified amount '{}' is not valid for this action \
                 abstraction, check 'allowed_amounts()' for more information",
                amount
            ))
        } else {
            self.amount = amount;
            Ok(())
        }
    }
}

impl std::fmt::Display for AbstractedRaise {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "raise {}", self.amount)
    }
}

impl AbstractedRaise {
    fn allowed_amounts(&self) -> &Vec<i32> {
        &self.amounts
    }
}
