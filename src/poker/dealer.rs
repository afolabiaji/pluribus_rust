use std::collections::HashSet;
use rand::seq::SliceRandom;

use super::deck::Deck;
use super::table::PokerTable;
use super::player::Player;
use super::card::Card;

pub struct Dealer {
    deck: Deck,
}

impl Dealer {
    pub fn new() -> Self {
        Self {
            deck: Deck::new(),
        }
    }

    pub fn deal_card(&mut self) -> Card {
        self.deck.pick(random: true)
    }

    pub fn deal_private_cards(&mut self, players: &mut Vec<Player>) {
        for _ in 0..2 {
            for player in players {
                let card = self.deal_card();
                player.add_private_card(card);
            }
        }
    }

    pub fn deal_community_cards(&mut self, table: &mut PokerTable, n_cards: usize) {
        if n_cards == 0 {
            panic!("Positive n of cards must be specified");
        }

        for _ in 0..n_cards {
            let card = self.deal_card();
            table.add_community_card(card);
        }
    }

    pub fn deal_flop(&mut self, table: &mut PokerTable) {
        self.deal_community_cards(table, 3);
    }

    pub fn deal_turn(&mut self, table: &mut PokerTable) {
        self.deal_community_cards(table, 1);
    }

    pub fn deal_river(&mut self, table: &mut PokerTable) {
        self.deal_community_cards(table, 1);
    }
}
