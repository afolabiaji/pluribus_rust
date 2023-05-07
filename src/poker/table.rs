use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::ptr;

use crate::poker::card::Card;
use crate::poker::dealer::Dealer;
use crate::poker::player::Player;
use crate::poker::pot::Pot;

pub struct PokerTable {
    players: Vec<Rc<Player>>,
    total_n_chips_on_table: i32,
    pot: Rc<Pot>,
    dealer: Rc<Dealer>,
    community_cards: Vec<Card>,
    n_games: i32,
}

impl PokerTable {
    pub fn new(players: Vec<Rc<Player>>, pot: Rc<Pot>, include_suits:Option<Vec<&'static str>>, include_ranks:Option<Vec<i32>>) -> Self {
        let total_n_chips_on_table = players.iter().map(|p| p.n_chips).sum();

        if players.len() < 2 {
            panic!("Must be at least two players on the table.");
        }

        if !players.iter().all(|p|{
            let borrowed_pot_from_player = p.pot.borrow();
            let borrowed_pot_rc = Rc::clone(&pot);
            borrowed_pot_rc.uid == borrowed_pot_from_player.uid
        }) {
            panic!("Players and table point to different pots.");
        }

        Self {
            players,
            total_n_chips_on_table,
            pot: pot.clone(),
            dealer: Rc::new(Dealer::new(include_suits, include_ranks)),
            community_cards: Vec::new(),
            n_games: 0,
        }
    }

    pub fn n_players(&self) -> usize {
        self.players.len()
    }

    pub fn set_players(&mut self, players: Vec<Rc<Player>>) {
        self.players = players;

        if !self.players.iter().all(|p|{
            let borrowed_pot_from_player = p.pot.borrow();
            let borrowed_pot_rc = Rc::clone(&self.pot);
            borrowed_pot_rc.uid == borrowed_pot_from_player.uid
        }){
            panic!("Players and table point to different pots.");
        }
    }

    pub fn add_community_card(&mut self, card: Card) {
        self.community_cards.push(card);
    }

    pub fn __repr__(&self) -> String {
        let player_names: Vec<&str> = self.players.iter().map(|p| p.name.as_str()).collect();
        format!("<PokerTable players={:?}>", player_names)
    }
}
