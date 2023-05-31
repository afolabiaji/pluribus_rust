use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::ptr;

use crate::poker::card::Card;
use crate::poker::dealer::Dealer;
use crate::poker::player::Player;
use crate::poker::pot::Pot;

pub struct PokerTable {
    pub players: Vec<Rc<RefCell<Player>>>,
    pub total_n_chips_on_table: i32,
    pub pot: Rc<RefCell<Pot>>,
    pub dealer: Rc<RefCell<Dealer>>,
    pub community_cards: Vec<Card>,
    pub n_games: i32,
}

impl PokerTable {
    pub fn new(players: Vec<Rc<RefCell<Player>>>, pot: Rc<RefCell<Pot>>, include_suits:Option<Vec<&'static str>>, include_ranks:Option<Vec<i32>>) -> Self {
        let total_n_chips_on_table = players.iter().map(|p|(*p.borrow()).n_chips).sum();

        if players.len() < 2 {
            panic!("Must be at least two players on the table.");
        }

        if !players.iter().all(|p|{
            let borrowed_pot_from_player = (*p.borrow()).pot.borrow();
            let borrowed_pot_rc = Rc::clone(&pot);
            borrowed_pot_rc.uid == borrowed_pot_from_player.uid
        }) {
            panic!("Players and table point to different pots.");
        }

        let new_dealer_refcell = RefCell::new(Dealer::new(include_suits, include_ranks));
        let new_dealer_rc = Rc::new(new_dealer_refcell);
        Self {
            players,
            total_n_chips_on_table,
            pot: pot.clone(),
            dealer: new_dealer_rc,
            community_cards: Vec::new(),
            n_games: 0,
        }
    }

    pub fn n_players(&self) -> usize {
        self.players.len()
    }

    pub fn set_players(&mut self, players: Vec<Rc<RefCell<Player>>>) {
        self.players = players;

        if !self.players.iter().all(|p|{
            let borrowed_pot_from_player = (*p.borrow()).pot.borrow();
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
        let player_names: Vec<&str> = self.players.iter().map(|p| (*p.borrow()).name.as_str()).collect();
        format!("<PokerTable players={:?}>", player_names)
    }
}
