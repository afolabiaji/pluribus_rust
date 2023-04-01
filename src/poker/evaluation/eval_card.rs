/// Static class that handles cards. We represent cards as 32-bit integers, so
/// there is no object instantiation - they are just ints. Most of the bits are
/// used, and have a specific meaning. See below:
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
use colored::*;

// the basics
pub const STR_RANKS: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];
pub const INT_RANKS: [i32; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
pub const PRIMES: [i32; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];
// hearts and diamonds
pub const PRETTY_REDS: [i32; 2] = [2, 4];

lazy_static! {
    // conversion from int => string
    pub static ref INT_SUIT_TO_CHAR_SUIT:HashMap<i32, char> = HashMap::from([
        (1, 's'),  // spades
        (2, 'h'),  // hearts
        (4, 'd'),  // diamonds
        (8, 'c'),  // clubs
    ]);

    // conversion from string => int
    pub static ref CHAR_SUIT_TO_INT_SUIT:HashMap<char, i32> = HashMap::from([
        ('s', 1),  // spades
        ('h', 2),  // hearts
        ('d', 4),  // diamonds
        ('c', 8),  // clubs
    ]);

    // for pretty printing
    pub static ref CHAR_RANK_TO_INT_RANK:HashMap<char, i32> = HashMap::from_iter(
        zip(STR_RANKS, INT_RANKS).collect::<Vec<_>>()
    );

    pub static ref PRETTY_SUITS:HashMap<i32, char> = HashMap::from([
        (1, '\u{2660}'),  // spades
        (2, '\u{2665}'),  // hearts
        (4, '\u{2666}'),  // diamonds
        (8, '\u{2663}'),  // clubs
    ]);
}

fn new(string: &str) -> i32 {
    let string_bytes: &[u8] = string.as_bytes();
    let rank_char: char = string_bytes[0] as char;

    let suit_char: char = string_bytes[1] as char;

    let rank_int: &i32 = CHAR_RANK_TO_INT_RANK
        .get(&rank_char)
        .unwrap();
    let suit_int: &i32 = CHAR_SUIT_TO_INT_SUIT
        .get(&suit_char)
        .unwrap();

    let rank_prime: i32 = PRIMES[*rank_int as usize];

    let bitrank = 1 << rank_int << 16;
    let suit = suit_int << 12;
    let rank = rank_int << 8;

    bitrank | suit | rank | rank_prime
}

pub fn int_to_str(card_int:i32) -> String {
    let rank_int = get_rank_int(card_int);
    let suit_int = get_suit_int(card_int);

    let str_rank = STR_RANKS[rank_int as usize];
    let char_suit = *INT_SUIT_TO_CHAR_SUIT.get(&suit_int).unwrap();

    format!("{}{}", str_rank, char_suit)
}

pub fn int_to_pretty_str(card_int:i32) -> ColoredString {
    let rank_int = get_rank_int(card_int);
    let suit_int = get_suit_int(card_int);

    let str_rank = STR_RANKS[rank_int as usize];
    let char_suit = *INT_SUIT_TO_CHAR_SUIT.get(&suit_int).unwrap();

    

    if PRETTY_REDS.contains(&suit_int){
        return format!("{}{}", str_rank, char_suit).red()
    } else{
        return format!("{}{}", str_rank, char_suit).red()
    }
}

fn get_rank_int(card_int:i32) -> i32{
    (card_int >> 8) & 0xF
}

fn get_suit_int(card_int:i32) -> i32{
    (card_int >> 12) & 0xF
}

fn get_bitrank_int(card_int:i32) -> i32{
    (card_int >> 16) & 0x1FFF
}

fn get_prme(card_int:i32) -> i32{
    card_int & 0x3F
}

fn hand_to_binary(card_strs:Vec<&str>) -> Vec<i32>{
    let mut bhand = Vec::new();

    for c in card_strs{
        bhand.push(new(c));
    }
        
    bhand
}

fn prime_product_from_hand(card_ints: Vec<i32>) -> i32 {
    let mut product = 1;
    for c in card_ints {
        product *= c & 0xFF;
    }
    product
}

fn prime_product_from_rankbits(rankbits: i32) -> i32 {
    // Returns the prime product using the bitrank (b)
    // bits of the hand. Each 1 in the sequence is converted
    // to the correct prime and multiplied in.
    // Params:
    //     rankbits = a single 32-bit (only 13-bits set) integer representing
    //             the ranks of 5 _different_ ranked cards
    //             (5 of 13 bits are set)
    // Primarily used for evaulating flushes and straights,
    // two occasions where we know the ranks are *ALL* different.
    // Assumes that the input is in form (set bits):
    //                         rankbits
    //                 +--------+--------+
    //                 |xxxbbbbb|bbbbbbbb|
    //                 +--------+--------+

    let mut product = 1;
    for i in INT_RANKS{
        if (rankbits & (1 << i)) != 0{
            product *= PRIMES[i as usize];
        }
    }
    product
}