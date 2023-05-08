/// Evaluates hand strengths using a variant of Cactus Kev's algorithm:
/// http://suffe.cool/poker/evaluator.html

/// I make considerable optimizations in terms of speed and memory usage,
/// in fact the lookup table generation can be done in under a second and
/// consequent evaluations are very fast. Won't beat C, but very fast as
/// all calculations are done with bit arithmetic and table lookups.
use std::rc::Rc;
use super::card::Card;

use super::lookup::{
    LookupTable,
    MaxHand
};

enum HandSize {
    Five,
    Six,
    Seven
}

struct Evaluator{
    table: LookupTable,
    // hand_size_map: HandSize, 
}

impl Evaluator {
    fn new() -> Evaluator{
        Evaluator {
            table: LookupTable::new(),
            // hand_size_map
        }
    }

    fn evaluate<T>(self, cards:Vec<Card>, board:Vec<Card>){
        let all_cards: Vec<i32> = cards.iter()
            .chain(board.iter())
            .map(|card| card.into())
            .collect();

        match all_cards.len() {
            5 => ,
            6,
            7
        }
        // return self.hand_size_map[len(all_cards)](all_cards)
    }

    fn _five(&self, cards:Vec<Card>):
        """
        Performs an evalution given cards in integer form, mapping them to
        a rank in the range [1, 7462], with lower ranks being more powerful.
        Variant of Cactus Kev's 5 card evaluator, though I saved a lot of memory
        space using a hash table and condensing some of the calculations.
        """
        # if flush
        if cards[0] & cards[1] & cards[2] & cards[3] & cards[4] & 0xF000:
            handOR = (cards[0] | cards[1] | cards[2] | cards[3] | cards[4]) >> 16
            prime = EvaluationCard.prime_product_from_rankbits(handOR)
            return self.table.flush_lookup[prime]

        # otherwise
        else:
            prime = EvaluationCard.prime_product_from_hand(cards)
            return self.table.unsuited_lookup[prime]
        
    fn get_rank_class(hr:i32) -> i32 {
        // Returns the class of hand from the hand hand_rank from evaluate.
        if (hr >= 0) && (hr <= LookupTable::MAX_STRAIGHT_FLUSH){
            LookupTable::MAX_TO_RANK_CLASS(MaxHand::StraightFlush)
        } else if hr <= LookupTable::MAX_FOUR_OF_A_KIND {
            LookupTable::MAX_TO_RANK_CLASS(MaxHand::FourOfAKind)
        }  else if hr <= LookupTable::MAX_FULL_HOUSE {
            LookupTable::MAX_TO_RANK_CLASS(MaxHand::FullHouse)
        } else if hr <= LookupTable::MAX_FLUSH {
            LookupTable::MAX_TO_RANK_CLASS(MaxHand::Flush)
        } else if hr <= LookupTable::MAX_STRAIGHT {
            LookupTable::MAX_TO_RANK_CLASS(MaxHand::Straight)
        } else if hr <= LookupTable::MAX_THREE_OF_A_KIND {
            LookupTable::MAX_TO_RANK_CLASS(MaxHand::ThreeOfAKind)
        } else if hr <= LookupTable::MAX_TWO_PAIR {
            LookupTable::MAX_TO_RANK_CLASS(MaxHand::TwoPair)
        } else if hr <= LookupTable::MAX_PAIR {
            LookupTable::MAX_TO_RANK_CLASS(MaxHand::OnePair)
        } else if hr <= LookupTable::MAX_HIGH_CARD {
            LookupTable::MAX_TO_RANK_CLASS(MaxHand::HighCard)
        } else {
            panic!("Inavlid hand rank, cannot return rank class")
        }
    }
        
}