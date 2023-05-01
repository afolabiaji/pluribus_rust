use std::collections::HashMap;

use crate::poker::card::Card;
use crate::poker::dealer::Dealer;
use crate::poker::player::Player;
use crate::poker::pot::Pot;

pub struct PokerTable<'a> {
    players: Vec<&'a mut Player<'a>>,
    total_n_chips_on_table: i32,
    pot: &'a mut Pot<'a>,
    dealer: Dealer<'a>,
    community_cards: Vec<Card>,
    n_games: i32,
}

impl<'a> PokerTable<'a> {
    pub fn new(players: Vec<&'a mut Player<'a>>, pot: &'a mut Pot<'a>, deck_kwargs: HashMap<String, String>) -> Self {
        let total_n_chips_on_table = players.iter().map(|p| p.n_chips).sum();

        if players.len() < 2 {
            panic!("Must be at least two players on the table.");
        }

        if !players.iter().all(|p| p.pot.uid == pot.uid) {
            panic!("Players and table point to different pots.");
        }

        Self {
            players,
            total_n_chips_on_table,
            pot,
            dealer: Dealer::new(deck_kwargs),
            community_cards: Vec::new(),
            n_games: 0,
        }
    }

    pub fn n_players(&self) -> usize {
        self.players.len()
    }

    pub fn set_players(&mut self, players: Vec<&'a mut Player<'a>>) {
        self.players = players;

        if !self.players.iter().all(|p| p.pot.uid == self.pot.uid) {
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
