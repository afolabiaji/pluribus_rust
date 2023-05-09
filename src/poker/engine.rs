use std::collections::{BTreeMap, HashMap};
use std::cmp::Ordering;
use std::rc::Rc;

use super::dealer::Dealer;
use super::evaluation::evaluator::Evaluator;
use super::player::Player;
use super::pot::{Pot, SidePot};
use super::state::PokerGameState;
use super::table::PokerTable;

use log::debug;

pub struct PokerEngine {
    table: RefCell<PokerTable>,
    small_blind: i32,
    big_blind: i32,
    evaluator: Evaluator,
    state: PokerGameState,
    wins_and_losses: Vec<(Rc<Player>, i32)>,
}

impl PokerEngine {
    pub fn new(table: Rc<PokerTable>, small_blind: i32, big_blind: i32) -> Self {
        PokerEngine {
            table,
            small_blind,
            big_blind,
            evaluator: Evaluator::new(),
            state: PokerGameState::new_hand(Rc::clone(&table)),
            wins_and_losses: Vec::new(),
        }
    }

    pub fn play_one_round(&mut self) {
        self.round_setup();
        self.all_dealing_and_betting_rounds();
        self.compute_winners();
        self.round_cleanup();
    }

    fn round_setup(&mut self) {
        self.table.pot.borrow_mut().reset();
        self.assign_order_to_players();
        self.assign_blinds();
    }

    fn all_dealing_and_betting_rounds(&mut self) {
        let players = &self.table.players;
        Dealer::deal_private_cards(players);
        self.betting_round(true);
        self.table.dealer.borrow_mut().deal_flop(RefCell::clone(&self.table));
        self.betting_round(false);
        self.table.dealer.borrow_mut().deal_turn(RefCell::clone(&self.table));
        self.betting_round(false);
        self.table.dealer.borrow_mut().deal_river(RefCell::clone(&self.table));
        self.betting_round(false);
    }

    fn compute_winners(&mut self) {
        let ranked_player_groups = self.rank_players_by_best_hand();
        let payouts = self.compute_payouts(ranked_player_groups);
        self.payout_players(payouts);
        debug!("Winnings computation complete. Players:");
        for player in &self.table.players {
            debug!("{:?}", player);
        }
    }

    fn round_cleanup(&mut self) {
        self.move_blinds();
    }

    fn get_players_in_pot(&self, player_group: &[Rc<Player>], pot: &SidePot) -> Vec<Rc<Player>> {
        let mut players_in_pot = Vec::new();
        for player in player_group.iter() {
            if pot.contains(player) {
                players_in_pot.push(Rc::clone(player));
            }
        }
        players_in_pot.sort_by(|a, b| a.order.cmp(&b.order));
        players_in_pot
    }

    // fn process_side_pot(&self, player_group: &[Rc<Player>], pot: &SidePot) -> HashMap<Rc<Player>, i32> {
    //     let mut payouts = HashMap::new();
    //     let players_in_pot = self.get_players_in_pot(player_group, pot);
    //     let n_players = players_in_pot.len();
    //     if n_players == 0 {
    //         return payouts;
    //     }
    //     let n_total = pot.total();
    //     let
    // }
}