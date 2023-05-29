use super::deck::Deck;
use super::table::PokerTable;
use super::player::Player;
use super::card::Card;

use std::cell::RefCell;

pub struct Dealer {
    deck: Deck,
}

impl Dealer {
    pub fn new(include_suits:Option<Vec<&'static str>>, include_ranks:Option<Vec<i32>>) -> Self {
        Self {
            deck: Deck::new(include_suits, include_ranks),
        }   
    }

    pub fn deal_card(&mut self) -> Card {
        self.deck.pick(true)
    }

    pub fn deal_private_cards(&mut self, players: Vec<RefCell<Player>>) {
        for _ in 0..2 {
            for player in &players {
                let card = self.deal_card();
                let mut mutable_player = player.borrow_mut();
                mutable_player.add_private_card(card);
            }
        }
    }

    pub fn deal_community_cards(&mut self, table: RefCell<PokerTable>, n_cards: usize) {
        if n_cards == 0 {
            panic!("Positive n of cards must be specified");
        }

        for _ in 0..n_cards {
            let card = self.deal_card();
            let mut mutable_table = table.borrow_mut();
            mutable_table.add_community_card(card);
        }
    }

    pub fn deal_flop(&mut self, table: RefCell<PokerTable>) {
        self.deal_community_cards(table, 3);
    }

    pub fn deal_turn(&mut self, table: RefCell<PokerTable>) {
        self.deal_community_cards(table, 1);
    }

    pub fn deal_river(&mut self, table: RefCell<PokerTable>) {
        self.deal_community_cards(table, 1);
    }
}
