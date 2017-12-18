extern crate generic_array;
extern crate rand;
extern crate sha3;

use self::sha3::{Digest, Sha3_256};
use self::generic_array::{typenum, GenericArray};
use self::rand::{OsRng, Rng};
use std::io::Write;

pub type HashResult = GenericArray<u8, typenum::U32>;

pub enum WriteDest {
    StdOut,
    StdErr,
    Filename(String),
}

pub fn hash(string: &str) -> HashResult {
    Sha3_256::digest(string.as_bytes())
}

pub fn hash_slice(slice: &[String]) -> HashResult {
    let mut hasher = Sha3_256::new();
    for each in slice {
        hasher.input(each.as_bytes());
    }

    hasher.result()
}

pub fn write(dest: &WriteDest, val: &str) {
    match dest {
        &WriteDest::StdOut => {
            let _ = ::std::io::stdout().write(val.as_bytes());
        }
        &WriteDest::StdErr => {
            let _ = ::std::io::stderr().write(val.as_bytes());
        }
        &WriteDest::Filename(ref c) => {
            let mut file = ::std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(c)
                .unwrap();

            let _ = file.write(val.as_bytes());
        }
    };
}

pub fn exit_with_message<T>(s: &str) -> T {
    write(&WriteDest::StdErr, &format!("{}\n", s));
    ::std::process::exit(1);
}

pub fn as_titlecase(s: &str) -> String {
    s.chars()
        .enumerate()
        .map(|(i, c)| {
            if i == 0 {
                format!("{}", c).to_uppercase()
            } else {
                format!("{}", c)
            }
        })
        .collect()
}

pub fn secure_rand_u8() -> u8 {
    let mut rng = match OsRng::new() {
        Ok(r) => r,
        Err(e) => panic!("failed to acquire OS rng, {}", e),
    };

    let val = rng.next_u32() as u8;

    val
}

pub fn combine_as_u32(a: u8, b: u8, c: u8, d: u8) -> u32 {
    a as u32 + ((b as u32).rotate_left(8)) + ((c as u32).rotate_left(16))
        + ((d as u32).rotate_left(24))
}

pub fn reduce_range(i: u32, max: usize) -> usize {
    (i as usize) % max
}
