use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;

use super::player::Player;

use std::cmp::{PartialEq, Eq};
use std::hash::{Hash, Hasher};

#[derive(Clone, Eq, PartialEq)]
pub struct Pot {
    pub pot: HashMap<String, i32>,
    pub uid: String,
}

impl Pot {
    pub fn new() -> Self {
        Pot {
            pot: HashMap::new(),
            uid: Uuid::new_v4().simple().to_string(),
        }
    }

    pub fn add_chips(&mut self, player: &String, n_chips: i32) {
        *self.pot.entry(*player).or_insert(0) += n_chips;
    }

    pub fn reset(&mut self) {
        self.pot.clear();
    }

    pub fn side_pots(&self) -> Vec<HashMap<String, i32>> {
        let mut side_pots = Vec::new();
        if self.pot.is_empty() {
            return side_pots;
        }
        let mut pot: HashMap<String, i32> = self.pot.clone();
        while !pot.is_empty() {
            side_pots.push(HashMap::new());
            let min_n_chips = *pot.values().min().unwrap();
            let mut players_to_pop = Vec::new();
            for (player, n_chips) in pot.iter_mut() {
                side_pots.last_mut().unwrap().insert(player.clone(), min_n_chips);
                *n_chips -= min_n_chips;
                if *n_chips == 0 {
                    players_to_pop.push(player.clone());
                }
            }
            for player in players_to_pop {
                pot.remove(&player);
            }
        }
        side_pots
    }

    pub fn uid(&self) -> &str {
        &self.uid
    }

    pub fn total(&self) -> i32 {
        self.pot.values().sum()
    }

    pub fn get_contribution(&self, player: &String) -> i32 {
        match self.pot.get(player) {
            Some(n_chips) => *n_chips,
            None => 0,
        }
    }
}

impl std::fmt::Debug for Pot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pot")
            .field("n_chips", &self.total())
            .finish()
    }
}

impl Hash for Pot {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uid.hash(state);
    }
}