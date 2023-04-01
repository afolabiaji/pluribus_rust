#[macro_use]
extern crate lazy_static;

pub mod poker;

use crate::poker::evaluation::eval_card::{
    CHAR_RANK_TO_INT_RANK
};
use crate::poker::evaluation::lookup::{
    MAX_TO_RANK_CLASS
};


fn main(){
    let byte1:i32 = 0xF000;
    let byte2:i32 = 0xF000;
    let bitwise_and:i32 = byte1 & byte2;
    println!("{:?}", CHAR_RANK_TO_INT_RANK.get(&"A").unwrap())
    // println!("{:?}", MAX_TO_RANK_CLASS.get(&10).unwrap());
    // println!("{:b}", byte1);
    // println!("{:b}", byte2);
    // println!("{:b}", bitwise_and);

}