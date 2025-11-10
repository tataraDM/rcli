use anyhow::Ok;

use crate::opts;
use rand::seq::SliceRandom;
use rand::{Rng, seq::IteratorRandom};
use zxcvbn::zxcvbn;

const SYMBOLS: &str = "!@#$%^&*_";
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &str = "0123456789";

pub fn process_genpass(opts: &opts::GenPassOpts) -> anyhow::Result<()> {
    let mut rng = rand::rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();
    if opts.lowercase {
        chars.extend_from_slice(LOWERCASE.as_bytes());
        password.push(LOWERCASE.chars().choose(&mut rng).unwrap());
    }

    if opts.uppercase {
        chars.extend_from_slice(UPPERCASE.as_bytes());
        password.push(UPPERCASE.chars().choose(&mut rng).unwrap());
    }

    if opts.number {
        chars.extend_from_slice(NUMBERS.as_bytes());
        password.push(NUMBERS.chars().choose(&mut rng).unwrap());
    }

    if opts.symbol {
        chars.extend_from_slice(SYMBOLS.as_bytes());
        password.push(SYMBOLS.chars().choose(&mut rng).unwrap());
    }

    for _ in 0..(opts.length - password.len()) {
        let idx = rng.random_range(0..chars.len());
        password.push(chars[idx] as char);
    }
    password.shuffle(&mut rng);
    let password = password.iter().collect::<String>();
    println!("{}", password);
    let result = zxcvbn(&password, &[]);
    println!("Strength: {}", result.score());
    Ok(())
}
