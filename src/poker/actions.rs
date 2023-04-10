pub const DUMMY_AMOUNTS: [i32; 6] = [10, 100, 500, 1000, 5000, 10000];

#[derive(Debug)]
pub struct Call;
impl std::fmt::Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "call")
    }
}

#[derive(Debug)]
pub struct Fold;
impl std::fmt::Display for Fold {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "fold")
    }
}

#[derive(Debug)]
pub struct Raise {
    amount:u32
}
impl std::fmt::Display for Raise {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "raise")
    }
}

#[derive(Debug)]
struct AbstractedRaise {
    amounts: Vec<u32>,
    amount: u32,
}

impl AbstractedRaise {
    fn new(allowed_amounts: Vec<u32>) -> Self {
        AbstractedRaise {
            amounts: allowed_amounts,
            amount: 0,
        }
    }

    fn call(&mut self, amount: u32) -> Result<(), String> {
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
    fn allowed_amounts(&self) -> &Vec<u32> {
        &self.amounts
    }
}
