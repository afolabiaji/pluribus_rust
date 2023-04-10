pub const DUMMY_AMOUNTS: [i32; 6] = [10, 100, 500, 1000, 5000, 10000];

pub struct Call;
impl std::fmt::Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "call")
    }
}

pub struct Fold;
impl std::fmt::Display for Fold {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "fold")
    }
}

pub struct Raise {
    amount:i32
}
impl std::fmt::Display for Raise {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "raise")
    }
}

pub struct AbstractedRaise {
    amounts: Vec<i32>,
    amount: i32,
}

impl AbstractedRaise {
    fn new(allowed_amounts: Vec<i32>) -> Self {
        AbstractedRaise {
            amounts: allowed_amounts,
            amount: 0,
        }
    }

    fn call(&mut self, amount: i32) -> Result<(), String> {
        if !self.amounts.contains(&amount) {
            return Err(format!(
                "Specified amount '{}' is not valid for this action abstraction, check 'allowed_amounts()' for more information",
                amount
            ));
        }
        self.amount = amount;
        Ok(())
    }

    fn allowed_amounts(&self) -> &Vec<i32> {
        &self.amounts
    }
}

impl std::fmt::Debug for AbstractedRaise {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "raise {}", self.amount)
    }
}
