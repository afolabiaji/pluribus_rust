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

// pub struct AbstractedRaise {
//     amounts: [i32; 6],
//     amount:Option<i32>
// }
// impl Default for AbstractedRaise {
//     fn default() -> AbstractedRaise {
//         AbstractedRaise {
//             amounts: DUMMY_AMOUNTS,
//         }
//     }
// }
// impl std::fmt::Display for AbstractedRaise {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "raise {}", self.amount)
//     }
// }