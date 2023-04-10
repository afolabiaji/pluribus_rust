use std::collections::HashSet;

struct Card {
    eval_card: EvaluationCard,
    rank: i32,
    suit: String,
}

impl Card {
    fn new(rank: &str, suit: &str) -> Self {
        let rank = match rank {
            "2" => 2,
            "3" => 3,
            "4" => 4,
            "5" => 5,
            "6" => 6,
            "7" => 7,
            "8" => 8,
            "9" => 9,
            "10" | "t" => 10,
            "j" => 11,
            "q" => 12,
            "k" => 13,
            "a" => 14,
            _ => panic!("Invalid rank"),
        };

        let suit = suit.to_lowercase();

        if !get_all_suits().contains(&suit) {
            panic!("Invalid suit");
        }

        let rank_char = rank_to_char(rank);

        let suit_char = suit.chars().next().unwrap();

        let eval_card = EvaluationCard::new(format!("{}{}", rank_char, suit_char).as_str());

        Card {
            eval_card,
            rank,
            suit,
        }
    }

    fn rank_to_str(rank: i32) -> &'static str {
        match rank {
            2 => "2",
            3 => "3",
            4 => "4",
            5 => "5",
            6 => "6",
            7 => "7",
            8 => "8",
            9 => "9",
            10 => "10",
            11 => "jack",
            12 => "queen",
            13 => "king",
            14 => "ace",
            _ => panic!("Invalid rank"),
        }
    }

    fn rank_to_char(rank: i32) -> &'static str {
        match rank {
            2 => "2",
            3 => "3",
            4 => "4",
            5 => "5",
            6 => "6",
            7 => "7",
            8 => "8",
            9 => "9",
            10 => "T",
            11 => "J",
            12 => "Q",
            13 => "K",
            14 => "A",
            _ => panic!("Invalid rank"),
        }
    }

    fn suit_to_icon(suit: &str) -> &'static str {
        match suit {
            "hearts" => "♥",
            "diamonds" => "♦",
            "clubs" => "♣",
            "spades" => "♠",
            _ => panic!("Invalid suit"),
        }
    }

    fn get_all_suits() -> HashSet<&'static str> {
        let suits: HashSet<&str> = ["spades", "diamonds", "clubs", "hearts"].iter().cloned().collect();
        suits
    }

    fn eval_card(&self) -> EvaluationCard {
        self.eval_card
    }

    fn rank_int(&self) -> i32 {
        self.rank
    }

    fn rank(&self) -> &'static str {
        Self::rank_to_str(self.rank)
    }

    fn suit(&self) -> &str {
        &self.suit
    }
}

impl std::fmt::Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let icon = Self::suit_to_icon(&self.suit);
        write!(f,"<Card card=[{} of {} {}]>",self.rank, self,suit, icon)
    }
}