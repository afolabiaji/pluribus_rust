#[macro_use]
extern crate lazy_static;

pub mod poker;
use crate::poker::evaluation::eval_card;

fn main(){
    println!("{:?}", eval_card::CHAR_RANK_TO_INT_RANK.get(&0).unwrap())
}