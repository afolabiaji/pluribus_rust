#[macro_use]
extern crate lazy_static;

pub mod poker;
use crate::poker::evaluation::eval_card;
use crate::poker::evaluation::lookup;


fn main(){
    println!("{:?}", lookup::MAX_TO_RANK_CLASS.get(&10).unwrap())
}