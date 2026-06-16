use std::{env, io};
use colored::*;

fn main(){
    let stdin = io::stdin();
    for line in stdin.lines(){
        print!("{} ", line.unwrap().red().bold());
    }
    let args = env::args().collect();
    dbg!(args)
}