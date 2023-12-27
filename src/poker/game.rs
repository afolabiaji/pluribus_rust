use std::rc::Rc;
use std::cell::RefCell;

use crate::poker::card::Card;
use crate::poker::deck::Deck;
use crate::poker::player::Player;
use crate::poker::pot::Pot;

pub struct PokerGame {
    pub players: Vec<Rc<RefCell<Player>>>,
    pub total_n_chips_on_table: i32,
    pub pot: Rc<RefCell<Pot>>,
    pub community_cards: Vec<Card>,
    pub n_games: i32,
    pub deck: Deck,
}

impl PokerGame {
    pub fn new(players: Vec<Rc<RefCell<Player>>>, pot: Rc<RefCell<Pot>>, include_suits:Option<Vec<&'static str>>, include_ranks:Option<Vec<i32>>) -> Self {
        let total_n_chips_on_table = players.iter().map(|p|p.borrow().n_chips).sum();

        if players.len() < 2 {
            panic!("Must be at least two players on the table.");
        }

        if !players.iter().all(|p|{
            let player = p.borrow_mut();
            let pot_ref = pot.borrow_mut();
            let borrowed_pot_from_player = player.pot.borrow_mut();
            pot_ref.uid == borrowed_pot_from_player.uid
        }) {
            panic!("Players and table point to different pots.");
        }

        Self {
            players,
            total_n_chips_on_table,
            pot: pot.clone(),
            community_cards: Vec::new(),
            n_games: 0,
            deck: Deck::new(include_suits, include_ranks),
        }
    }

    pub fn n_players(&self) -> usize {
        self.players.len()
    }

    pub fn set_players(&mut self) {
        if !self.players.iter().all(|p|{
            let player = p.borrow_mut();
            let pot_ref = self.pot.borrow_mut();
            let borrowed_pot_from_player = player.pot.borrow();

            pot_ref.uid == borrowed_pot_from_player.uid
        }){
            panic!("Players and table point to different pots.");
        }
    }

    pub fn add_community_card(&mut self, card: Card) {
        self.community_cards.push(card);
    }

    pub fn deal_card(&mut self) -> Card {
        self.deck.pick(true)
    }

    pub fn deal_private_cards(&mut self) {
        let cloned_players: Vec<_> = self.players.to_vec();
        for _ in 0..2 {
            for player in &cloned_players {
                let card = self.deal_card();
                let mut mutable_player = player.borrow_mut();
                mutable_player.add_private_card(card);
            }
        }
    }

    pub fn deal_community_cards(&mut self, n_cards: usize) {
        if n_cards == 0 {
            panic!("Positive n of cards must be specified");
        }

        for _ in 0..n_cards {
            let card = self.deal_card();
            self.add_community_card(card);
        }
    }

    pub fn deal_flop(&mut self) {
        self.deal_community_cards(3);
    }

    pub fn deal_turn(&mut self) {
        self.deal_community_cards(1);
    }

    pub fn deal_river(&mut self) {
        self.deal_community_cards(1);
    }

    pub fn __repr__(&self) -> String {
        let player_names: Vec<String> = self.players.iter().map(|p| {
            let player = p.borrow();
            player.name.clone()
        }).collect();
        format!("<PokerTable players={:?}>", player_names)
    }
}
