/// Number of Distinct Hand Values:
/// Straight Flush   10
/// Four of a Kind   156      [(13 choose 2) * (2 choose 1)]
/// Full Houses      156      [(13 choose 2) * (2 choose 1)]
/// Flush            1277     [(13 choose 5) - 10 straight flushes]
/// Straight         10
/// Three of a Kind  858      [(13 choose 3) * (3 choose 1)]
/// Two Pair         858      [(13 choose 3) * (3 choose 2)]
/// One Pair         2860     [(13 choose 4) * (4 choose 1)]
/// High Card      + 1277     [(13 choose 5) - 10 straights]
/// -------------------------
/// TOTAL            7462
/// Here we create a lookup table which maps:
///     5 card hand's unique prime product => rank in range [1, 7462]
/// Examples:
/// * Royal flush (best hand possible)          => 1
/// * 7-5-4-3-2 unsuited (worst hand possible)  => 7462
use std::collections::HashMap;
use super::eval_card::{
    EvaluationCard
};
use std::fs::File;
use std::io::{BufWriter, Write};
use itertools::Itertools;


pub enum MaxHand {
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}
pub struct LookupTable {
    pub flush_lookup: HashMap<i32, i32>,
    pub unsuited_lookup: HashMap<i32, i32>,
}

impl LookupTable {
    pub const MAX_STRAIGHT_FLUSH:i32 = 10;
    pub const MAX_FOUR_OF_A_KIND:i32 = 166;
    pub const MAX_FULL_HOUSE:i32 = 322;
    pub const MAX_FLUSH:i32 = 1599;
    pub const MAX_STRAIGHT:i32 = 1609;
    pub const MAX_THREE_OF_A_KIND:i32 = 2467;
    pub const MAX_TWO_PAIR:i32 = 3325;
    pub const MAX_PAIR:i32 = 6185;
    pub const MAX_HIGH_CARD:i32 = 7462;

    // conversion from int => string
    pub fn MAX_TO_RANK_CLASS(rank: MaxHand) -> i32 {
        match rank {
            MaxHand::StraightFlush => 1,
            MaxHand::FourOfAKind => 2,
            MaxHand::FullHouse => 3,
            MaxHand::Flush => 4,
            MaxHand::Straight => 5,
            MaxHand::ThreeOfAKind => 6,
            MaxHand::TwoPair => 7,
            MaxHand::OnePair => 8,
            MaxHand::HighCard => 9
        }
    }

    pub fn RANK_CLASS_TO_STRING(class_int:i32) -> &'static str {
        match class_int {
            1 => "Straight Flush",
            2 => "Four of a Kind",
            3 => "Full House",
            4 => "Flush",
            5 => "Straight",
            6 => "Three of a Kind",
            7 => "Two Pair",
            8 => "Pair",
            9 => "High Card",
            _ => panic!("class int not in dictionary")
        }
    }
    
    pub fn new() -> LookupTable {
        let mut lookup_table:LookupTable = LookupTable {
            flush_lookup: HashMap::new(),
            unsuited_lookup: HashMap::new(),
        };
        lookup_table.flushes();
        lookup_table.multiples();
        lookup_table
    }

    fn flushes(&mut self){
        // Straight flushes and flushes.

        // Lookup is done on 13 bit integer (2^13 > 7462):
        // xxxbbbbb bbbbbbbb => integer hand index
        

        // straight flushes in rank order
        let straight_flushes: Vec<i32> = Vec::from([
            7936,  // int('0b1111100000000', 2), // royal flush
            3968,  // int('0b111110000000', 2),
            1984,  // int('0b11111000000', 2),
            992,  // int('0b1111100000', 2),
            496,  // int('0b111110000', 2),
            248,  // int('0b11111000', 2),
            124,  // int('0b1111100', 2),
            62,  // int('0b111110', 2),
            31,  // int('0b11111', 2),
            4111,  // int('0b1000000001111', 2) // 5 high
        ]);

        // now we'll dynamically generate all the other
        // flushes (including straight flushes)
        let mut flushes = Vec::new();
        let intval = i32::from_str_radix("0b11111", 2).unwrap();
        let mut gen = LookupTable::get_lexographically_next_bit_sequence(intval);

        // 1277 = number of high cards
        // 1277 + len(str_flushes) is number of hands with all cards unique rank
        let flush_len = 0..(1277 + straight_flushes.len() - 1);
        for _ in flush_len {
            // we also iterate over SFs
            // pull the next flush pattern from our generator
            let f = gen.next().unwrap();
            
            // if this flush matches perfectly any
            // straight flush, do not add it
            let mut not_sf: bool = true;
            for sf in &straight_flushes {
                // if f XOR sf == 0, then bit pattern
                // is same, and we should not add
                if (f ^ sf) == 0 {
                    not_sf = false
                };
            }
                

            if not_sf {
                flushes.push(f)
            };
        }
        // we started from the lowest straight pattern, now we want to start
        // ranking from the most powerful hands, so we reverse
        flushes.reverse();
        // now add to the lookup map:
        // start with straight flushes and the rank of 1
        // since it is the best hand in poker
        // rank 1 = Royal Flush!
        LookupTable::fill_in_lookup_table(
            1,
            &straight_flushes,
            &mut self.flush_lookup
        );
        // we start the counting for flushes on max full house, which
        // is the worst rank that a full house can have (2,2,2,3,3)
        LookupTable::fill_in_lookup_table(
            LookupTable::MAX_FULL_HOUSE + 1,
            &flushes,
            &mut self.flush_lookup
        );
        // we can reuse these bit sequences for straights
        // and high cards since they are inherently related
        // and differ only by context
        self.straight_and_highcards(&straight_flushes, &flushes);
            
    }

    fn fill_in_lookup_table(rank_init: i32, rankbits_list: &Vec<i32>, lookup_table: &mut HashMap<i32, i32>) {
        let mut rank = rank_init;
        for rb in rankbits_list{
            let prime_product = EvaluationCard::prime_product_from_rankbits(*rb);
            lookup_table.insert(prime_product, rank);
            rank += 1;
        }
    }

    fn multiples(&mut self) {
        let backwards_ranks = (0..EvaluationCard::INT_RANKS.len())
            .rev()
            .collect::<Vec<usize>>();

        // 1) Four of a Kind
        let mut rank = LookupTable::MAX_STRAIGHT_FLUSH + 1;

        // for each choice of a set of four rank
        for i in &backwards_ranks {
            // and for each possible kicker rank
            let mut kickers = backwards_ranks.clone();
            kickers.remove(*i);
            for k in &kickers {
                let product =
                    EvaluationCard::PRIMES[*i].pow(4) * EvaluationCard::PRIMES[*k];
                self.unsuited_lookup.insert(product, rank);
                rank += 1;
            }
        }

        // 2) Full House
        rank = LookupTable::MAX_FOUR_OF_A_KIND + 1;

        // for each three of a kind
        for i in &backwards_ranks {
            // and for each choice of pair rank
            let mut pairranks = backwards_ranks.clone();
            pairranks.remove(*i);
            for pr in &pairranks {
                let product = EvaluationCard::PRIMES[*i].pow(3)
                    * EvaluationCard::PRIMES[*pr].pow(2);
                self.unsuited_lookup.insert(product, rank);
                rank += 1;
            }
        }

        // 3) Three of a Kind
        rank = LookupTable::MAX_STRAIGHT + 1;

        // pick three of one rank
        for r in &backwards_ranks {
            let mut kickers = backwards_ranks.clone();
            kickers.remove(*r);
            let gen = kickers.iter().combinations(2);

            for kickers in gen {
                let c1 = kickers[0];
                let c2 = kickers[1];
                let product = EvaluationCard::PRIMES[*r].pow(3)
                    * EvaluationCard::PRIMES[*c1]
                    * EvaluationCard::PRIMES[*c2];
                self.unsuited_lookup.insert(product, rank);
                rank += 1;
            }
        }

        // 4) Two Pair
        rank = LookupTable::MAX_THREE_OF_A_KIND + 1;

        let tpgen = backwards_ranks.iter().combinations(2);

        for tp in tpgen {

            let pair1 = tp[0];
            let pair2 = tp[1];
            let mut kickers = backwards_ranks.clone();
            kickers.retain(|&x| x != *pair1 && x != *pair2);
            for &kicker in kickers.iter() {

                let product =
                    EvaluationCard::PRIMES[*pair1].pow(2)
                    * EvaluationCard::PRIMES[*pair2].pow(2)
                    * EvaluationCard::PRIMES[kicker];
                self.unsuited_lookup.insert(product, rank);
                rank += 1;
            }
        }
        // 5) Pair
        let mut rank = LookupTable::MAX_TWO_PAIR + 1;

        // choose a pair
        for pairrank in backwards_ranks.iter() {
            let mut kickers = backwards_ranks.clone();
            kickers.retain(|&x| x != *pairrank);
            let kgen = kickers.iter().combinations(3);

            for kickers in kgen {
                let (k1, k2, k3) = (kickers[0], kickers[1], kickers[2]);
                let product = 
                    EvaluationCard::PRIMES[*pairrank].pow(2)
                    * EvaluationCard::PRIMES[*k1] 
                    * EvaluationCard::PRIMES[*k2] 
                    * EvaluationCard::PRIMES[*k3];
                self.unsuited_lookup.insert(product, rank);
                rank += 1;
            }
        }
    }

    fn straight_and_highcards(&mut self, straights: &Vec<i32>, highcards: &Vec<i32>) {
        LookupTable::fill_in_lookup_table(
            LookupTable::MAX_FLUSH + 1,
            straights,
            &mut self.unsuited_lookup
        );
        LookupTable::fill_in_lookup_table(
            LookupTable::MAX_PAIR + 1,
            highcards,
            &mut self.unsuited_lookup
        );
    }
    

    
    
    fn write_table_to_disk(table: &HashMap<u64, i32>, filepath: &str) -> std::io::Result<()> {
        let file = File::create(filepath)?;
        let mut writer = BufWriter::new(file);
        for (prime_prod, rank) in table {
            let line = format!("{},{}\n", prime_prod, rank);
            writer.write(line.as_bytes())?;
        }
        writer.flush()?;
        Ok(())
    }

    fn get_lexographically_next_bit_sequence(bits: i32) -> impl Iterator<Item = i32> {
        let t = (bits | (bits - 1)) + 1;
        let mut next = t | ((((t & -t) / (bits & -bits)) >> 1) - 1);
        std::iter::from_fn(move || {
            let result = next;
            let t = (next | (next - 1)) + 1;
            next = t | ((((t & -t) / (next & -next)) >> 1) - 1);
            Some(result)
        })
    }
                
}