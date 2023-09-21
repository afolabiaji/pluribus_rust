use uuid::Uuid;

use std::cell::RefCell;
use super::actions::{Action, Call, Fold, Raise};
use super::card::Card;
use super::pot::Pot;
use super::state::PokerGameState;

use std::cmp::{PartialEq, Eq};
use std::hash::{Hash, Hasher};
use std::fmt;
use std::rc::Rc;

#[derive(Clone, Eq, PartialEq)]
pub struct Player {
    pub name: String,
    pub n_chips: i32,
    pub cards: Vec<Card>,
    pub id: String,
    pub pot: Rc<RefCell<Pot>>,
    pub order: Option<u32>,
    pub is_small_blind: bool,
    pub is_big_blind: bool,
    pub is_dealer: bool,
    _is_active: bool,
}

impl Player {
    pub fn new(name: String, initial_chips: i32, pot: Rc<RefCell<Pot>>) -> Player {
        Player {
            name,
            n_chips: initial_chips,
            cards: Vec::new(),
            _is_active: true,
            id: Uuid::new_v4().simple().to_string(),
            pot,
            order: None,
            is_small_blind: false,
            is_big_blind: false,
            is_dealer: false,
        }
    }

    pub fn add_chips(&mut self, chips: i32) {
        self.n_chips += chips;
    }

    pub fn fold(&mut self) -> Box<dyn Action> {
        self._is_active = false;
        Box::new(Fold {})
    }

    pub fn call(&mut self, players: &Vec<Rc<RefCell<Player>>>) -> Box<dyn Action> {
        if self.is_all_in() {
            return Box::new(Call {});
        } else {
            let biggest_bet = players
                .iter()
                .filter(|p| {
                    let player = p.borrow();
                    player.is_active()
                })
                .map(|p| {
                    let player = p.borrow();
                    player.n_bet_chips()
                })
                .max()
                .unwrap_or_default();
            let n_chips_to_call = biggest_bet - self.n_bet_chips();
            self.add_to_pot(n_chips_to_call);
            Box::new(Call {})
        }
    }

    pub fn raise_to(&mut self, n_chips: i32) -> Box<dyn Action> {
        let n_chips = self.add_to_pot(n_chips);
        let mut raise_action = Raise::new();
        raise_action.set_amount(n_chips);
        Box::new(raise_action)
    }

    fn try_to_make_full_bet(&mut self, mut n_chips: i32) -> i32 {
        if self.n_chips - n_chips < 0 {
            n_chips = self.n_chips;
        }
        n_chips
    }

    pub fn add_to_pot(&mut self, n_chips: i32) -> i32 {
        if n_chips < 0 {
            panic!("Can not subtract chips from pot.")
        }
        let n_chips = self.try_to_make_full_bet(n_chips);
        let mut mutable_pot = self.pot.borrow_mut();
        mutable_pot.add_chips(&self.id, n_chips);
        self.n_chips -= n_chips;
        n_chips
    }

    pub fn add_private_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn take_action(&mut self, _game_state: &PokerGameState) -> PokerGameState {
        unimplemented!("All poker strategy is implemented here.");
    }

    pub fn is_active(&self) -> bool {
        // Getter for if the player is playing or not.
        self._is_active
    }

    pub fn set_active(&mut self, is_active: bool) {
        // Setter for if the player is playing or not.
        self._is_active = is_active;
    }

    pub fn is_all_in(&self) -> bool {
        // Return if the player is all in or not.
        self._is_active && self.n_chips == 0
    }

    pub fn n_bet_chips(&self) -> i32 {
        // Returns the n_chips this player has bet so far.
        let borrowed_pot = self.pot.borrow();
        *borrowed_pot.pot.get(&self.id).unwrap()
    }
}

impl Hash for Player {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let folded = !self.is_active();
        write!(
            f,
            "<Player name=\"{}\" n_chips={:05} n_bet_chips={:05} folded={}>",
            self.name, self.n_chips, self.n_bet_chips(), folded
        )
    }
}