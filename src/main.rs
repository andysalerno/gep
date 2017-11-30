extern crate sha3;
extern crate generic_array;
extern crate rand;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate rpassword;

mod dict_reader;
mod cli_opt;

use structopt::StructOpt;
use sha3::{Sha3_256, Digest};
use generic_array::{GenericArray, typenum};
use rand::{Rng, OsRng};
use dict_reader::DictReader;
use cli_opt::CliOpt;

type HashResult = GenericArray<u8, typenum::U32>;

const PASS_PROMPT: &'static str = "Master password: ";
const PASS_CONFIRM: &'static str = "Confirm master password: ";

const DEFAULT_DICT_PATH: &'static str = "dictionary.txt";
const NUM_DELIM: &'static str = ":";
const WORD_DELIM: &'static str = "";
const PASS_WORD_LEN: usize = 4;
const PASS_RETRY_ATTEMPTS: u8 = 3;

fn main() {
    let opt = CliOpt::from_args();

    let salt_num = 
        if opt.rand_num { Some(secure_rand_u8()) }
        else { opt.num };

    let master_password = prompt_master_password()
        .expect("Failed to retrieve master password. Quiting.");

    let precursor = build_precursor(&master_password,
                                    &opt.site,
                                    opt.username.as_ref(),
                                    salt_num);

    if opt.verbose {
        output(&format!("Precursor: {}", precursor));
    }

    let hashed = hash(&precursor);

    if opt.hex {
        output(&format!("{:x}", hashed));
    }
    else {
        let result = build_password_from_hash(hashed, salt_num, opt.dict.as_ref());
        output(&result);
    }
}

fn build_precursor(master_password: &str, site: &str, username: Option<&String>, salt_num: Option<u8>) -> String {
    let mut combine = String::new();

    combine.push_str(master_password);

    combine.push_str(site);

    if let Some(u) = username {
        combine.push_str(u);
    }

    if let Some(n) = salt_num {
        combine.push_str(NUM_DELIM);
        combine.push_str(&n.to_string());
    }

    return combine;
}

fn build_password_from_hash(hash: HashResult, salt_num: Option<u8>, dict: Option<&String>) -> String {
    let mut words: [String; PASS_WORD_LEN] = Default::default();

    let dict_reader = match dict {
        Some(dict_path) => DictReader::new(dict_path.clone()),
        None => DictReader::new(String::from(DEFAULT_DICT_PATH)),
    };

    let dict_len = dict_reader.len();

    for i in 0..PASS_WORD_LEN {
        let v = i * 4;
        let couplet = combine_as_u32(
            hash[v], hash[v+1], hash[v+2], hash[v+3]
        );

        let word = dict_reader.get_nth_word(reduce_range(couplet, dict_len));
        let word_title_cased = as_titlecase(word);
        words[i] = String::from(word_title_cased);
    }

    let number = match salt_num {
        Some(n) => n,
        None => *hash.last().unwrap()
    };

    let joined_words = words.join(WORD_DELIM);

    return format!("{}{}{}", joined_words, NUM_DELIM, number);
}

fn reduce_range(i: u32, max: usize) -> usize {
    (i as usize) % max
}

fn combine_as_u32(a: u8, b: u8, c: u8, d: u8) -> u32 {
    a as u32 +
        ((b as u32).rotate_left(8)) +
        ((c as u32).rotate_left(16)) +
        ((d as u32).rotate_left(24))
}

fn as_titlecase(s: &str) -> String {
    s.chars().enumerate()
        .map(|(i, c)| 
            if i == 0 { format!("{}", c).to_uppercase() }
            else { format!("{}", c) }).collect()
}

fn prompt_master_password() -> Result<String, &'static str> {
    for _ in 0..PASS_RETRY_ATTEMPTS {

        let password = rpassword::prompt_password_stdout(PASS_PROMPT)
            .expect("unable to prompt stdout for password");

        let password_confirm = rpassword::prompt_password_stdout(PASS_CONFIRM)
            .expect("unable to prompt stdout for password");

        if password == password_confirm {
            return Ok(password);
        }
        else {
            output("Confirmation did not match. Try again.");
        }
    }

    return Err("password mismatch failure");
}

fn hash(string: &str) -> HashResult {
    Sha3_256::digest(string.as_bytes())
}

fn output(s: &str) {
    println!("{}", s);
}

fn secure_rand_u8() -> u8 {
    let mut rng = match OsRng::new() {
       Ok(r) => r,
       Err(e) => panic!("failed to acquire OS rng, {}", e),
    };

    let val = rng.next_u32() as u8;

    return val;
}