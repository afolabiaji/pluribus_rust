use super::PokerTable;
use std::fmt;

#[derive(Debug, Clone)]
pub struct PokerGameState<'a> {
    previous_state: Option<&'a PokerGameState<'a>>,
    table: &'a PokerTable,
    player: Option<usize>,
    action: Option<&'a str>,
    is_terminal: bool,
}

impl<'a> PokerGameState<'a> {
    pub fn new_hand(table: &'a PokerTable) -> Self {
        PokerGameState {
            previous_state: None,
            table,
            player: None,
            action: None,
            is_terminal: false,
        }
    }

    pub fn table(&self) -> &'a PokerTable {
        self.table
    }

    pub fn is_terminal(&self) -> bool {
        self.is_terminal
    }

    pub fn is_chance_node(&self) -> bool {
        false
    }

    pub fn current_player(&self) -> Option<usize> {
        self.player
    }

    pub fn utility(&self, winner_i: usize) -> Vec<f64> {
        if self.is_terminal {
            let mut utility = vec![-1.0; self.table.n_players()];
            utility[winner_i] = 1.0;
            utility
        } else {
            vec![0.0; self.table.n_players()]
        }
    }
}

