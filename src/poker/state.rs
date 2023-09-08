use super::table::PokerTable;
use super::player::Player;

use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;


pub struct PokerGameState {
    previous_state: Option<Rc<PokerGameState>>,
    table: PokerTable,
    player: Option<Rc<Player>>,
    action: Option<String>,
    is_terminal: bool,
}

impl PokerGameState {
    pub fn new_hand(table: PokerTable) -> Self {
        PokerGameState {
            previous_state: None,
            table,
            player: None,
            action: None,
            is_terminal: false,
        }
    }

    // pub fn table(&self) -> PokerTable {
    //     self.table
    // }

    pub fn is_terminal(&self) -> bool {
        self.is_terminal
    }

    pub fn is_chance_node(&self) -> bool {
        false
    }

    pub fn current_player(&self) -> Option<Rc<Player>> {
        self.player.clone()
    }

    // pub fn utility(&self, winner_i: Rc<Player>) -> Vec<f64> {
    //     if self.is_terminal {
    //         let mut utility = vec![-1.0; self.table.n_players()];
    //         utility[winner_i] = 1.0;
    //         utility
    //     } else {
    //         vec![0.0; self.table.n_players()]
    //     }
    // }
}

