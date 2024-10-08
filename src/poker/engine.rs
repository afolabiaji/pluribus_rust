use std::collections::{ HashMap };
use std::rc::Rc;
use std::cell::RefCell;

use super::card::Card;
use super::evaluation::evaluator::Evaluator;
use super::player::Player;
use super::game::PokerGame;

pub struct PokerEngine {
    game: PokerGame,
    small_blind: i32,
    big_blind: i32,
    evaluator: Evaluator,
    // state: PokerGameState,
    wins_and_losses: Vec<(Rc<RefCell<Player>>, i32)>,
}

impl PokerEngine {
    pub fn new(game: PokerGame, small_blind: i32, big_blind: i32) -> Self {
        PokerEngine {
            game,
            small_blind,
            big_blind,
            evaluator: Evaluator::new(),
            // state: PokerGameState::new_hand(&game),
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
        self.reset_pot();
        self._assign_order_to_players();
        self._assign_blinds();
    }

    fn all_dealing_and_betting_rounds(&mut self) {
        self.game.deal_private_cards();
        self._betting_round(true);
        self.game.deal_flop();
        self._betting_round(false);
        self.game.deal_turn();
        self._betting_round(false);
        self.game.deal_river();
        self._betting_round(false);
    }

    fn compute_winners(&mut self) {
        let ranked_player_groups = self._rank_players_by_best_hand();
        let payouts = self._compute_payouts(&ranked_player_groups);
        self.payout_players(&payouts);
        println!("Winnings computation complete. Players:");
        for player in &self.game.players {
            let p = player.borrow();
            println!("{}", { p });
        }
    }

    fn round_cleanup(&mut self) {
        self.move_blinds();
    }

    fn _get_players_in_pot(
        &self,
        player_group: &Vec<Rc<RefCell<Player>>>,
        pot: &HashMap<String, i32>
    ) -> Vec<Rc<RefCell<Player>>> {
        let mut players_in_pot = Vec::new();
        for player in player_group.iter() {
            let p = player.borrow();
            if pot.contains_key(&p.id) {
                players_in_pot.push(Rc::clone(player));
            }
        }
        players_in_pot.sort_by(|a, b| {
            let a = a.borrow();
            let b = b.borrow();
            a.order.cmp(&b.order)
        });
        players_in_pot
    }

    fn _process_side_pot(
        &self,
        player_group: &Vec<Rc<RefCell<Player>>>,
        pot: &HashMap<String, i32>
    ) -> Result<HashMap<String, i32>, &'static str> {
        let mut payouts: HashMap<String, i32> = HashMap::new();
        let players_in_pot = self._get_players_in_pot(player_group, pot);
        let n_players = players_in_pot.len() as i32;
        if n_players == 0 {
            return Ok(HashMap::new());
        }
        let n_total: i32 = pot.values().cloned().sum();
        let n_per_player = n_total / n_players;
        let n_remainder = n_total - n_players * n_per_player;
        for player in players_in_pot.iter() {
            let p = player.borrow();
            let entry = payouts.entry(p.id.clone()).or_insert(0);
            *entry += n_per_player;
        }
        for i in 0..n_remainder {
            let player = &players_in_pot[i as usize];
            let p = player.borrow();
            let entry = payouts.entry(p.id.clone()).or_insert(0);
            *entry += 1;
        }
        Ok(payouts)
    }

    fn _compute_payouts(
        &self,
        ranked_player_groups: &Vec<Vec<Rc<RefCell<Player>>>>
    ) -> HashMap<Rc<RefCell<Player>>, i32> {
        let payouts: HashMap<Rc<RefCell<Player>>, i32> = HashMap::new();
        let borrowed_pot = self.game.pot.borrow_mut();
        for pot in borrowed_pot.side_pots() {
            for player_group in ranked_player_groups {
                let pot_payouts = self._process_side_pot(player_group, &pot);
                if let Ok(mut payouts) = pot_payouts {
                    let cloned_payouts = payouts.clone();
                    for (player_id, winnings) in cloned_payouts {
                        let p_id = player_id.clone();
                        *payouts.entry(p_id).or_insert(0) += winnings;
                    }
                    break;
                }
            }
        }
        payouts
    }

    fn reset_pot(&mut self) {
        let mut borrowed_pot = self.game.pot.borrow_mut();
        borrowed_pot.reset();
    }

    fn payout_players(&mut self, payouts: &HashMap<Rc<RefCell<Player>>, i32>) {
        self.reset_pot();
        for (player, winnings) in payouts {
            let mut p = player.borrow_mut();
            p.add_chips(*winnings);
        }
    }

    fn _rank_players_by_best_hand(&self) -> Vec<Vec<Rc<RefCell<Player>>>> {
        let game_cards: Vec<Card> = self.game.community_cards.clone();
        let mut grouped_players: HashMap<i32, Vec<Rc<RefCell<Player>>>> = HashMap::new();
        for player in &self.game.players {
            let borrowed_player = player.borrow_mut();
            if borrowed_player.is_active() {
                let hand_cards: Vec<Card> = borrowed_player.cards.clone();
                let rank = self.evaluator.evaluate(&game_cards, &hand_cards);
                let hand_class = self.evaluator.get_rank_class(rank);
                let hand_desc = self.evaluator.class_to_string(hand_class).to_lowercase();
                println!("Rank #{} {} {}", rank, borrowed_player, hand_desc);
                grouped_players.entry(rank).or_insert(Vec::new()).push(Rc::clone(player));
            }
        }
        let mut ranked_player_groups: Vec<Vec<Rc<RefCell<Player>>>> = Vec::new();
        let mut ranks: Vec<i32> = grouped_players.keys().cloned().collect();
        ranks.sort();
        for rank in &ranks {
            ranked_player_groups.push(grouped_players.get(rank).unwrap().clone());
        }
        ranked_player_groups
    }

    fn _assign_order_to_players(&self) {
        for (player_i, player) in self.game.players.iter().enumerate() {
            let mut borrowed_player = player.borrow_mut();
            borrowed_player.order = Some(player_i as u32);
        }
    }

    fn _assign_blinds(&self) {
        let mut borrowed_player_0 = self.game.players[0].borrow_mut();
        let mut borrowed_player_1 = self.game.players[1].borrow_mut();
        borrowed_player_0.add_to_pot(self.small_blind);
        borrowed_player_1.add_to_pot(self.big_blind);
        // println!("Assigned blinds to players {}", {self.game.borrow()}.players[0..2]);
    }

    fn move_blinds(&mut self) {
        let players = &mut self.game.players;
        players.rotate_left(1);
        // println!("Rotated players from {} to {}", self.game.players.into_iter().map(|x| x.borrow()).collect(), players.into_iter().map(|x| x.borrow()).collect());
        self.game.set_players();
    }

    fn _players_in_order_of_betting(&self, first_round: bool) -> Vec<Rc<RefCell<Player>>> {
        if first_round {
            let mut players = self.game.players[2..].to_vec();
            players.extend_from_slice(&self.game.players[..2]);
            players
        } else {
            self.game.players.clone()
        }
    }

    fn _all_active_players_take_action(&mut self, first_round: bool) {
        for player in self._players_in_order_of_betting(first_round) {
            let borrowed_player = player.borrow_mut();
            if borrowed_player.is_active() {
                // self.state = borrowed_player.take_action(&self.state);
            }
        }
    }

    fn _bet_until_everyone_has_bet_evenly(&mut self) {
        let mut first_round = true;
        println!("Started round of betting.");
        while first_round || self.more_betting_needed() {
            self._all_active_players_take_action(first_round);
            first_round = false;
            println!("> Betting iter, total: {}", self.all_bets().iter().sum::<i32>());
        }
    }

    fn _betting_round(&mut self, _first_round: bool) {
        if self.n_players_with_moves() > 1 {
            self._bet_until_everyone_has_bet_evenly();
            println!(
                "Finished round of betting, {} active players, {} all in players.",
                self.n_active_players(),
                self.n_all_in_players()
            );
        } else {
            println!("Skipping betting as no players are free to bet.");
        }
        self._post_betting_analysis();
    }

    fn _post_betting_analysis(&self) {
        let borrowed_pot = {
            self.game.pot.borrow()
        };
        println!("Pot at the end of betting: {:?}", borrowed_pot);
        println!("Players at the end of betting:");
        for player in &self.game.players {
            println!("{}", player.borrow());
        }
        let total_n_chips =
            borrowed_pot.total() +
            self.game.players
                .iter()
                .map(|p| {
                    let player = p.borrow();
                    player.n_chips
                })
                .sum::<i32>();

        let n_chips_correct = total_n_chips == self.game.total_n_chips_on_table;
        let pot_correct =
            borrowed_pot.total() ==
            ({
                self.game.players
                    .iter()
                    .map(|p| {
                        let player = p.borrow();
                        player.n_chips
                    })
                    .sum::<i32>()
            });
        if !n_chips_correct || !pot_correct {
            panic!("Bad logic - total n_chips are not the same as at the start of the game");
        }
    }

    fn n_players_with_moves(&self) -> i32 {
        self.game.players
            .iter()
            .filter(|p| {
                let player = p.borrow();
                player.is_active() && !player.is_all_in()
            })
            .count() as i32
    }

    fn n_active_players(&self) -> i32 {
        self.game.players
            .iter()
            .filter(|p| {
                let player = p.borrow();
                player.is_active()
            })
            .count() as i32
    }

    fn n_all_in_players(&self) -> i32 {
        self.game.players
            .iter()
            .filter(|p| {
                let player = p.borrow();
                player.is_active() && player.is_all_in()
            })
            .count() as i32
    }

    fn all_bets(&self) -> Vec<i32> {
        self.game.players
            .iter()
            .map(|p| {
                let player = p.borrow();
                player.n_bet_chips()
            })
            .collect()
    }

    fn more_betting_needed(&self) -> bool {
        let active_complete_bets: Vec<i32> = self.game.players
            .iter()
            .filter(|p| {
                let player = p.borrow();
                player.is_active() && !player.is_all_in()
            })
            .map(|p| {
                let player = p.borrow();
                player.n_bet_chips()
            })
            .collect();
        let all_bets_equal = active_complete_bets.iter().all(|&x| x == active_complete_bets[0]);
        !all_bets_equal
    }
}
