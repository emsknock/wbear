use rand::seq::IndexedRandom;
use wbear_macro::include_quotelist;

const QUOTES: [&str; 556] = include_quotelist!(include_str!("./quotes.txt"));

fn main() {
    let pick = QUOTES
        .choose(&mut rand::rng())
        .expect("quote list should not be empty");

    println!();
    println!("{pick}");
    println!("     \\");
    println!("    ʕ•ﻌ•ʔ");
}
