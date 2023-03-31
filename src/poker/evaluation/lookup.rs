///Number of Distinct Hand Values:
///Straight Flush   10
///Four of a Kind   156      [(13 choose 2) * (2 choose 1)]
///Full Houses      156      [(13 choose 2) * (2 choose 1)]
///Flush            1277     [(13 choose 5) - 10 straight flushes]
///Straight         10
///Three of a Kind  858      [(13 choose 3) * (3 choose 1)]
///Two Pair         858      [(13 choose 3) * (3 choose 2)]
///One Pair         2860     [(13 choose 4) * (4 choose 1)]
///High Card      + 1277     [(13 choose 5) - 10 straights]
///-------------------------
///TOTAL            7462
///Here we create a lookup table which maps:
///    5 card hand's unique prime product => rank in range [1, 7462]
///Examples:
///* Royal flush (best hand possible)          => 1
///* 7-5-4-3-2 unsuited (worst hand possible)  => 7462
use std::collections::HashMap;

pub const MAX_STRAIGHT_FLUSH:i32 = 10;
pub const MAX_FOUR_OF_A_KIND:i32 = 166;
pub const MAX_FULL_HOUSE:i32 = 322;
pub const MAX_FLUSH:i32 = 1599;
pub const MAX_STRAIGHT:i32 = 1609;
pub const MAX_THREE_OF_A_KIND:i32 = 2467;
pub const MAX_TWO_PAIR:i32 = 3325;
pub const MAX_PAIR:i32 = 6185;
pub const MAX_HIGH_CARD:i32 = 7462;

lazy_static! {
    // conversion from int => string
    pub static ref MAX_TO_RANK_CLASS:HashMap<i32, i32> = HashMap::from([
        (MAX_STRAIGHT_FLUSH, 1),  
        (MAX_FOUR_OF_A_KIND, 2),  
        (MAX_FULL_HOUSE, 3),  
        (MAX_FLUSH, 4),  
        (MAX_STRAIGHT, 5),  
        (MAX_THREE_OF_A_KIND, 6),  
        (MAX_TWO_PAIR, 7),  
        (MAX_PAIR, 8),  
        (MAX_HIGH_CARD, 9),  
    ]);

    pub static ref RANK_CLASS_TO_STRING:HashMap<i32, &'static str> = HashMap::from([
        (1, "Straight Flush"),
        (2, "Four of a Kind"),
        (3, "Full House"),
        (4, "Flush"),
        (5, "Straight"),
        (6, "Three of a Kind"),
        (7, "Two Pair"),
        (8, "Pair"),
        (9, "High Card"),
    ]);
}