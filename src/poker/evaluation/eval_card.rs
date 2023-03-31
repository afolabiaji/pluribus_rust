/// Static class that handles cards. We represent cards as 32-bit integers, so
/// there is no object instantiation - they are just ints. Most of the bits are
///used, and have a specific meaning. See below:
///                                EvaluationCard:
///                      bitrank     suit rank   prime
///                +--------+--------+--------+--------+
///                |xxxbbbbb|bbbbbbbb|cdhsrrrr|xxpppppp|
///                +--------+--------+--------+--------+
///    1) p = prime number of rank (deuce=2,trey=3,four=5,...,ace=41)
///    2) r = rank of card (deuce=0,trey=1,four=2,five=3,...,ace=12)
///    3) cdhs = suit of card (bit turned on based on suit of card)
///    4) b = bit turned on depending on rank of card
///    5) x = unused
/// This representation will allow us to do very important things like:
/// - Make a unique prime prodcut for each hand
/// - Detect flushes
/// - Detect straights
/// and is also quite performant.

use std::collections::HashMap;
use std::iter::zip;


// the basics
pub const STR_RANKS: &str = "23456789TJQKA";
pub const INT_RANKS: [i32; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
pub const PRIMES: [i32; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];
pub const INT_SUIT_TO_CHAR_SUIT: &str = "xshxdxxxc";
// hearts and diamonds
pub const PRETTY_REDS: [i32;2] = [2, 4];

lazy_static! {
    // conversion from string => int
    pub static ref CHAR_SUIT_TO_INT_SUIT:HashMap<&'static str, i32> = HashMap::from([
        ("s", 1),  // spades
        ("h", 2),  // hearts
        ("d", 4),  // diamonds
        ("c", 8),  // clubs
    ]);

    // for pretty printing
    pub static ref CHAR_RANK_TO_INT_RANK:HashMap<i32, i32> = HashMap::from_iter(
        zip(INT_RANKS, PRIMES).collect::<Vec<_>>()
    );

    pub static ref PRETTY_SUITS:HashMap<i32, &'static str> = HashMap::from([
        (1, "\u{2660}"),  // spades
        (2, "\u{2665}"),  // hearts
        (4, "\u{2666}"),  // diamonds
        (8, "\u{2663}"),  // clubs
    ]);
}