use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::poker::card::Card;

use std::vec::Vec;

let hs = Card::get_all_suits();
static DEFAULT_INCLUDE_SUITS: [&str] = Vec::from_iter(hs);
static DEFAULT_INCLUDE_RANKS: &[i32] = &[2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];

pub struct Deck {
    include_suits: Vec<&'static str>,
    include_ranks: Vec<i32>,
    cards_in_deck: Vec<Card>,
    dealt_cards: Vec<Card>,
}

impl Deck {
    pub fn new(include_suits: Option<Vec<&'static str>>, include_ranks: Option<Vec<i32>>) -> Deck {
        let suits = include_suits.unwrap_or_else(|| DEFAULT_INCLUDE_SUITS.to_vec());
        let ranks = include_ranks.unwrap_or_else(|| DEFAULT_INCLUDE_RANKS.to_vec());
        let mut cards_in_deck = Vec::new();
        for suit in suits.iter() {
            for rank in ranks.iter() {
                cards_in_deck.push(Card::new(*rank, *suit));
            }
        }
        let mut rng = thread_rng();
        cards_in_deck.shuffle(&mut rng);
        Deck {
            include_suits: suits,
            include_ranks: ranks,
            cards_in_deck,
            dealt_cards: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.cards_in_deck.len() + self.dealt_cards.len()
    }

    pub fn reset(&mut self) {
        self.cards_in_deck = Vec::new();
        for suit in self.include_suits.iter() {
            for rank in self.include_ranks.iter() {
                self.cards_in_deck.push(Card::new(*rank, *suit));
            }
        }
        let mut rng = thread_rng();
        self.cards_in_deck.shuffle(&mut rng);
        self.dealt_cards = Vec::new();
    }

    pub fn pick(&mut self, random: bool) -> Card {
        if self.cards_in_deck.is_empty() {
            panic!("Deck is empty - please use Deck::reset()");
        }
        let index = if random {
            thread_rng().gen_range(0, self.cards_in_deck.len())
        } else {
            self.cards_in_deck.len() - 1
        };
        let card = self.cards_in_deck.remove(index);
        self.dealt_cards.push(card.clone());
        card
    }

    pub fn remove(&mut self, card: &Card) {
        if let Some(index) = self.cards_in_deck.iter().position(|c| c == card) {
            self.cards_in_deck.remove(index);
            self.dealt_cards.push(card.clone());
        }
    }
}
