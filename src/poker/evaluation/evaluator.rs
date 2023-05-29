/// Evaluates hand strengths using a variant of Cactus Kev's algorithm:
/// http://suffe.cool/poker/evaluator.html

/// I make considerable optimizations in terms of speed and memory usage,
/// in fact the lookup table generation can be done in under a second and
/// consequent evaluations are very fast. Won't beat C, but very fast as
/// all calculations are done with bit arithmetic and table lookups.

use itertools::Itertools;

use super::super::card::Card;

use super::lookup::{
    LookupTable,
    MaxHand
};
use super::eval_card::EvaluationCard;

enum HandSize {
    Five,
    Six,
    Seven
}

pub struct Evaluator{
    table: LookupTable,
}

impl Evaluator {
    pub fn new() -> Evaluator{
        Evaluator {
            table: LookupTable::new(),
        }
    }

    pub fn evaluate(&self, cards:&Vec<Card>, board:&Vec<Card>) -> i32{
        let all_cards: Vec<i32> = cards.iter()
            .chain(board.iter())
            .map(|card| (*card).into())
            .collect();

        match all_cards.len() {
            5 => self._five(all_cards),
            6 => self._six(all_cards),
            7 => self._seven(all_cards),
        }
    }

    fn _five(&self, cards:Vec<i32>) -> i32 {
        // if flush
        if cards[0] & cards[1] & cards[2] & cards[3] & cards[4] & 0xF000 != 0 {
            let hand_or = (cards[0] | cards[1] | cards[2] | cards[3] | cards[4]) >> 16;
            let prime = EvaluationCard::prime_product_from_rankbits(hand_or);
            *self.table.flush_lookup.get(&prime).unwrap()
        } else {
            let prime = EvaluationCard::prime_product_from_hand(cards);
            *self.table.unsuited_lookup.get(&prime).unwrap()
        }
    }

    fn _six(&self, cards:Vec<i32>) -> i32 {
        let mut minimum = LookupTable::MAX_HIGH_CARD;

        for combo in cards.iter().combinations(5){
            let combo = combo.iter().map(|&x| x.clone()).collect();
            let score = self._five(combo);
            if score < minimum {
                minimum = score;
            }
        } 

        minimum
    }
            
    fn _seven(&self, cards:Vec<i32>) -> i32 {
        let mut minimum = LookupTable::MAX_HIGH_CARD;

        for combo in cards.iter().combinations(5){
            let combo = combo.iter().map(|&x| x.clone()).collect();
            let score = self._five(combo);
            if score < minimum {
                minimum = score;
            }
        } 

        minimum
    }
        
    pub fn get_rank_class(&self, hr:i32) -> i32 {
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
    pub fn class_to_string(&self, class_int: i32) -> &'static str {
        LookupTable::RANK_CLASS_TO_STRING(class_int)
    }

    pub fn get_five_card_rank_percentage(&self, hand_rank: i32) -> f32 {
        hand_rank as f32 / LookupTable::MAX_HIGH_CARD as f32
    }

    pub fn hand_summary(&self, board: &[Card], hands: &[Vec<Card>]) {
        assert_eq!(board.len(), 5, "Invalid board length");
        for hand in hands {
            assert_eq!(hand.len(), 2, "Invalid hand length");
        }
    
        let line_length = 10;
        let stages = ["FLOP", "TURN", "RIVER"];
    
        for i in 0..stages.len() {
            let line = "=".repeat(line_length);
            println!("{} {} {}", line, stages[i], line);
    
            let mut best_rank = 7463;
            let mut winners = vec![];
            for (player, hand) in hands.iter().enumerate() {
                let rank = self.evaluate(hand, &board[..(i + 3)].to_vec());
                let rank_class = self.get_rank_class(rank);
                let class_string = self.class_to_string(rank_class);
                let percentage =
                    1.0 - self.get_five_card_rank_percentage(rank); // higher better here
                println!(
                    "Player {} hand = {}, percentage rank among all hands = {}",
                    player + 1,
                    class_string,
                    percentage
                );
    
                // detect winner
                if rank == best_rank {
                    winners.push(player);
                    best_rank = rank;
                } else if rank < best_rank {
                    winners = vec![player];
                    best_rank = rank;
                }
            }
    
            // if we're not on the river
            if i != stages.iter().position(|&s| s == "RIVER").unwrap() {
                if winners.len() == 1 {
                    println!(
                        "Player {} hand is currently winning.\n",
                        winners[0] + 1
                    );
                } else {
                    println!(
                        "Players {:?} are tied for the lead.\n",
                        winners.iter().map(|&p| p + 1).collect::<Vec<_>>()
                    );
                }
            // otherwise on all other streets
            } else {
                let hand_result = self.class_to_string(
                    self.get_rank_class(self.evaluate(&hands[winners[0]], &board.to_vec()))
                );
                println!();
                println!("{} HAND OVER {}", line, line);
                if winners.len() == 1 {
                    println!(
                        "Player {} is the winner with a {}\n",
                        winners[0] + 1,
                        hand_result
                    );
                } else {
                    println!(
                        "Players {:?} tied for the win with a {}\n",
                        winners.iter().map(|&p| p + 1).collect::<Vec<_>>(),
                        hand_result
                    );
                }
            }
        }
    }
}