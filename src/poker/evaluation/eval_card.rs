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
use std::fmt::Display;
use std::iter::zip;
use colored::*;

pub struct EvaluationCard;

impl EvaluationCard{
    // the basics
    pub const STR_RANKS: [char; 13] = [
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];
    pub const INT_RANKS: [i32; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    pub const PRIMES: [i32; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];
    // hearts and diamonds
    pub const PRETTY_REDS: [i32; 2] = [2, 4];

    // conversion from int => string
    pub fn INT_SUIT_TO_CHAR_SUIT() -> HashMap<i32, char> { 
        HashMap::from([
            (1, 's'),  // spades
            (2, 'h'),  // hearts
            (4, 'd'),  // diamonds
            (8, 'c'),  // clubs
        ])
    }

    // conversion from string => int
    pub fn CHAR_SUIT_TO_INT_SUIT() -> HashMap<char, i32> {
        HashMap::from([
            ('s', 1),  // spades
            ('h', 2),  // hearts
            ('d', 4),  // diamonds
            ('c', 8),  // clubs
        ])
    }

    // for pretty printing
    pub fn CHAR_RANK_TO_INT_RANK() -> HashMap<char, i32> { 
        HashMap::from_iter(
            zip(EvaluationCard::STR_RANKS, EvaluationCard::INT_RANKS).collect::<Vec<_>>()
        )
    }

    pub fn PRETTY_SUITS() -> HashMap<i32, char> {
        HashMap::from([
            (1, '\u{2660}'),  // spades
            (2, '\u{2665}'),  // hearts
            (4, '\u{2666}'),  // diamonds
            (8, '\u{2663}'),  // clubs
        ])
    }
    

    pub fn new(string: &str) -> i32 {
        let string_bytes: &[u8] = string.as_bytes();
        let rank_char: char = string_bytes[0] as char;

        let suit_char: char = string_bytes[1] as char;

        let rank_binding = EvaluationCard::CHAR_RANK_TO_INT_RANK();
        let rank_int: &i32 = rank_binding.get(&rank_char)
            .unwrap_or_else(|| { panic!("{}", EvaluationCard::generate_panic_string(&rank_char, "CHAR_RANK_TO_INT_RANK")) });
        
        let suit_binding = EvaluationCard::CHAR_SUIT_TO_INT_SUIT();
        let suit_int: &i32 = suit_binding.get(&suit_char)
            .unwrap_or_else(|| { panic!("{}", EvaluationCard::generate_panic_string(&suit_char, "CHAR_SUIT_TO_INT_SUIT")) });
                

        let rank_prime: i32 = EvaluationCard::PRIMES[*rank_int as usize];

        let bitrank = 1 << rank_int << 16;
        let suit = suit_int << 12;
        let rank = rank_int << 8;

        bitrank | suit | rank | rank_prime
    }

    fn generate_panic_string<T: Display>(key:&T, h_map:&str) -> String {
        format!("This key: {} does not exist in {}", key, h_map)
    }

    pub fn int_to_str(card_int:i32) -> String {
        let rank_int = EvaluationCard::get_rank_int(card_int);
        let suit_int = EvaluationCard::get_suit_int(card_int);

        let str_rank = EvaluationCard::STR_RANKS[rank_int as usize];
        let binding = EvaluationCard::INT_SUIT_TO_CHAR_SUIT();
        let char_suit = *binding.get(&suit_int)
            .unwrap_or_else(|| { panic!("{}", EvaluationCard::generate_panic_string(&suit_int, "INT_SUIT_TO_CHAR_SUIT")) });

        format!("{}{}", str_rank, char_suit)
    }

    pub fn int_to_pretty_str(card_int:i32) -> ColoredString {
        let rank_int = EvaluationCard::get_rank_int(card_int);
        let suit_int = EvaluationCard::get_suit_int(card_int);

        let str_rank = EvaluationCard::STR_RANKS[rank_int as usize];
        let binding = EvaluationCard::INT_SUIT_TO_CHAR_SUIT();
        let char_suit = *binding.get(&suit_int)
            .unwrap_or_else(|| { panic!("{}", EvaluationCard::generate_panic_string(&suit_int, "INT_SUIT_TO_CHAR_SUIT")) });

        

        if EvaluationCard::PRETTY_REDS.contains(&suit_int){
            format!("{}{}", str_rank, char_suit).red()
        } else{
            format!("{}{}", str_rank, char_suit).black()
        }
    }

    fn get_rank_int(card_int:i32) -> i32{
        (card_int >> 8) & 0xF
    }

    fn get_suit_int(card_int:i32) -> i32{
        (card_int >> 12) & 0xF
    }


    fn _print_pretty_card(card_int:i32){
        println!("{}", EvaluationCard::int_to_pretty_str(card_int));
    }

    pub fn print_pretty_cards(card_ints:Vec<i32>){
        let mut output = String::from(" ");

        for i in 0..card_ints.len(){
            let c = card_ints[i];
            output.push_str(&EvaluationCard::int_to_pretty_str(c));
            if i != card_ints.len() - 1{
                output.push(',');
            }else{
                output.push(' ');
            }
        }

        println!("{output}");
    }
    fn _get_bitrank_int(card_int:i32) -> i32{
        (card_int >> 16) & 0x1FFF
    }

    fn _get_prme(card_int:i32) -> i32{
        card_int & 0x3F
    }

    fn _hand_to_binary(card_strs:Vec<&str>) -> Vec<i32>{
        let mut bhand = Vec::new();

        for c in card_strs{
            bhand.push(EvaluationCard::new(c));
        }
            
        bhand
    }

    pub fn prime_product_from_hand(card_ints: Vec<i32>) -> i32 {
        let mut product = 1;
        for c in card_ints {
            product *= c & 0xFF;
        }
        product
    }

    pub fn prime_product_from_rankbits(rankbits: i32) -> i32 {
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
        for i in EvaluationCard::INT_RANKS{
            if (rankbits & (1 << i)) != 0{
                product *= EvaluationCard::PRIMES[i as usize];
            }
        }
        product
    }
}

