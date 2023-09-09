    use std::borrow::Borrow;
    use std::collections::{HashMap};
    use std::rc::Rc;
    use std::cell::RefCell;

    use super::card::Card;
    use super::evaluation::evaluator::Evaluator;
    use super::player::{Player, self};
    use super::pot::Pot;
    use super::state::PokerGameState;
    use super::game::PokerGame;


    pub struct PokerEngine {
        game: PokerGame,
        small_blind: i32,
        big_blind: i32,
        evaluator: Evaluator,
        // state: PokerGameState,
        wins_and_losses: Vec<(Rc<Player>, i32)>,
    }

    impl PokerEngine {
        pub fn new(game: PokerGame, small_blind: i32, big_blind: i32) -> Self {
            PokerEngine {
                game: game,
                small_blind,
                big_blind,
                evaluator: Evaluator::new(),
                // state: PokerGameState::new_hand(table),
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
            self.assign_order_to_players();
            self.assign_blinds();
        }

        fn all_dealing_and_betting_rounds(&mut self) {
            self.game.deal_private_cards();
            self.betting_round(true);
            self.game.deal_flop();
            self.betting_round(false);
            self.game.deal_turn();
            self.betting_round(false);
            self.game.deal_river();
            self.betting_round(false);
            // let mut dealer = &self.table.dealer.borrow_mut();
            
        }

        fn compute_winners(&mut self) {
            let ranked_player_groups = self.rank_players_by_best_hand();
            let payouts = self._compute_payouts(ranked_player_groups);
            self.payout_players(&payouts);
            println!("Winnings computation complete. Players:");
            for &player in &self.game.players {
                println!("{}", {&player});
            }
        }

        fn round_cleanup(&mut self) {
            self.move_blinds();
        }

        fn _get_players_in_pot(&self, player_group: Vec<Rc<Player>>, pot: &HashMap<String, i32>) -> Vec<Rc<Player>> {
            let mut players_in_pot = Vec::new();
            for player in player_group.iter() {
                if pot.contains_key(&{*player}.id) {
                    players_in_pot.push(Rc::clone(&player));
                }
            }
            players_in_pot.sort_by(|a, b| a.order.cmp(&b.order));
            players_in_pot
        }

        fn _process_side_pot(&self, player_group: Vec<Rc<Player>>, pot: HashMap<String, i32>) -> Result<HashMap<String, i32>, &'static str> {
            let mut payouts: HashMap<String, i32> = HashMap::new();
            let players_in_pot = self._get_players_in_pot(player_group, &pot);
            let n_players = players_in_pot.len() as i32;
            if n_players == 0 {
                return Ok(HashMap::new());
            }
            let n_total: i32 = pot.values().cloned().sum();
            let n_per_player = n_total / n_players;
            let n_remainder = n_total - n_players * n_per_player;
            for player in players_in_pot.iter() {
                let entry = payouts.entry(player.id).or_insert(0);
                *entry += n_per_player;
            }
            for i in 0..n_remainder {
                let player = players_in_pot[i as usize];
                let entry = payouts.entry(player.id).or_insert(0);
                *entry += 1;
            }
            Ok(payouts)
        }

        fn _compute_payouts(&self, ranked_player_groups: Vec<Vec<Rc<Player>>>) -> HashMap<Rc<Player>, i32>{
            let mut payouts: HashMap<Rc<Player>, i32> = HashMap::new();
            let mut borrowed_pot = self.table.pot.borrow_mut();
            for pot in borrowed_pot.side_pots() {
                for player_group in ranked_player_groups {
                    let pot_payouts = self._process_side_pot(player_group, pot);
                    if let Ok(mut payouts) = pot_payouts {
                        for (player_id, winnings) in payouts {
                            *payouts.entry(player_id).or_insert(0) += winnings;
                        }
                        break;
                    }
                }
            }
            payouts
        }

        fn reset_pot(&mut self) {
            self.game.pot.reset();
        }
        fn payout_players(&self, payouts: &HashMap<Rc<Player>, i32>) {
            // let mut self.table = {*self.table}.borrow_mut();
            // let mut borrowed_pot = {*self.table.pot}.borrow_mut();
            // borrowed_pot.reset();
            self.reset_pot();
            for (player, winnings) in payouts {
                player.add_chips(*winnings);
            }
        }
        
        fn rank_players_by_best_hand(&self) -> Vec<Vec<Rc<Player>>> {
            let table_cards: Vec<Card> = self.table.community_cards;
            let mut grouped_players: HashMap<i32, Vec<Rc<Player>>> = HashMap::new();
            for player in self.table.players {
                let mut borrowed_player = {*player}.borrow_mut();
                if borrowed_player.is_active() {
                    let hand_cards: Vec<Card> = borrowed_player.cards;
                    let rank = self.evaluator.evaluate(&table_cards, &hand_cards);
                    let hand_class = self.evaluator.get_rank_class(rank);
                    let hand_desc = self.evaluator.class_to_string(hand_class).to_lowercase();
                    println!("Rank #{} {} {}", rank, borrowed_player, hand_desc);
                    grouped_players.entry(rank).or_insert(Vec::new()).push(Rc::new(*borrowed_player));
                }
            }
            let mut ranked_player_groups: Vec<Vec<Rc<Player>>> = Vec::new();
            let mut ranks: Vec<i32> = grouped_players.keys().cloned().collect();
            ranks.sort();
            for rank in ranks {
                ranked_player_groups.push(grouped_players.get(&rank).unwrap().clone());
            }
            ranked_player_groups
        }


        fn assign_order_to_players(&self) {
            for (player_i, player) in self.table.players.iter().enumerate() {
                {{**player}.borrow()}.order = Some(player_i);
            }
        }

        fn assign_blinds(&self) {
            let mut borrowed_player_0 = self.table.players[0].borrow_mut();
            let mut borrowed_player_1 = self.table.players[1].borrow_mut();
            borrowed_player_0.add_to_pot(self.small_blind);
            borrowed_player_1.add_to_pot(self.big_blind);
            // println!("Assigned blinds to players {}", {self.table.borrow()}.players[0..2]);
        }

        fn move_blinds(&self) {
            let mut players = self.table.players.clone();
            players.push(players.remove(0));
            // println!("Rotated players from {} to {}", self.table.players.into_iter().map(|x| x.borrow()).collect(), players.into_iter().map(|x| x.borrow()).collect());
            self.table.set_players(players);
        }

        fn players_in_order_of_betting(&self, first_round: bool) -> Vec<Rc<RefCell<Player>>> {
            if first_round {
                let mut players = self.table.players[2..].to_vec();
                players.extend_from_slice(&self.table.players[..2]);
                players
            } else {
                self.table.players.clone()
            }
        }

        fn all_active_players_take_action(&mut self, first_round: bool) {
            for player in self.players_in_order_of_betting(first_round) {
                let mut borrwoed_player = {*player}.borrow_mut();
                if borrwoed_player.is_active() {
                    self.state = borrwoed_player.take_action(&self.state);
                }
            }
        }

        fn bet_until_everyone_has_bet_evenly(&mut self) {
            let mut first_round = true;
            println!("Started round of betting.");
            while first_round || self.more_betting_needed() {
                self.all_active_players_take_action(first_round);
                first_round = false;
                println!("> Betting iter, total: {}", self.all_bets().iter().sum::<i32>());
            }
        }
        
        fn betting_round(&mut self, first_round: bool) {
            if self.n_players_with_moves() > 1 {
                self.bet_until_everyone_has_bet_evenly();
                println!("Finished round of betting, {} active players, {} all in players.",
                    self.n_active_players(), self.n_all_in_players());
            } else {
                println!("Skipping betting as no players are free to bet.");
            }
            self.post_betting_analysis();
        }
        
        fn post_betting_analysis(&self) {
            let borrowed_pot = {*self.table.pot}.borrow();
            println!("Pot at the end of betting: {:?}", borrowed_pot);
            println!("Players at the end of betting:");
            for player in self.table.players {
                println!("{}", {*player}.borrow());
            }
            let total_n_chips = 
                borrowed_pot.total() 
                +   self.table.players
                    .iter()
                    .map(|p| {
                        let player = {**p}.borrow();
                        player.n_chips
                    })
                    .sum::<i32>();

            let n_chips_correct = total_n_chips == self.table.total_n_chips_on_table;
            let pot_correct = borrowed_pot.total() == {
                self.table.players
                .iter()
                .map(|p| {
                    let player = {**p}.borrow();
                    player.n_chips
                })
                .sum::<i32>()};
            if !n_chips_correct || !pot_correct {
                panic!("Bad logic - total n_chips are not the same as at the start of the game");
            }
        }
        
        fn n_players_with_moves(&self) -> i32 {
            self.table.players.iter().filter(|p|{
                let player = {***p}.borrow();
                player.is_active() && !player.is_all_in()
            }).count() as i32
        }
        
        fn n_active_players(&self) -> i32 {
            self.table.players.iter().filter(|p| {
                let player = {***p}.borrow();
                player.is_active()
            }).count() as i32
        }
        
        fn n_all_in_players(&self) -> i32 {
            self.table.players.iter().filter(|p| {
                let player = {***p}.borrow();
                player.is_active() && player.is_all_in()
            }).count() as i32
        }
        
        fn all_bets(&self) -> Vec<i32> {
            self.table.players.iter().map(|p| {
                let player = {**p}.borrow();
                player.n_bet_chips()
            }).collect()
        }
        
        fn more_betting_needed(&self) -> bool {
            let active_complete_bets: Vec<i32> = self.table.players.iter()
                .filter(|p| {
                    let player = {***p}.borrow();
                    player.is_active() && !player.is_all_in()
                })
                .map(|p| {
                    let player = {**p}.borrow();
                    player.n_bet_chips()
                })
                .collect();
            let all_bets_equal = active_complete_bets.iter().all(|&x| x == active_complete_bets[0]);
            !all_bets_equal
        }


    }






