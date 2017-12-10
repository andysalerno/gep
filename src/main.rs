extern crate rpassword;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

mod dict_reader;
mod cli_opt;
mod config;
mod util;

use structopt::StructOpt;
use dict_reader::DictReader;
use cli_opt::CliOpt;

fn main() {
    let opt = CliOpt::from_args();

    if opt.debug {
        util::output(config::DEBUG_WARNING);
    }

    let master_password = prompt_master_password()
        .unwrap_or_else(|_| util::exit_with_message(config::EXCEEDED_ATTEMPTS));

    let salt_num = if opt.rand_num {
        util::secure_rand_u8()
    } else if let Some(n) = opt.num {
        n
    } else {
        let master_hashed = util::hash(&master_password);
        let collected_byte = master_hashed[0];

        collected_byte as u8
    };

    let precursor = build_precursor(&master_password, &opt.site, opt.username.as_ref(), salt_num);

    if opt.debug {
        util::output(&format!("Precursor: {}", precursor));
    }

    let hashed = util::hash(&precursor);

    if opt.hex {
        // user requests the hex'd result only,
        // don't build from dictionary
        util::output(&format!("{:x}", hashed));
    } else {
        let dict_reader = match opt.dict {
            Some(dict_path) => DictReader::new(dict_path),
            None => DictReader::new(String::from(config::DEFAULT_DICT_PATH)),
        };

        let wordvec = dict_reader.get_wordvec();

        if opt.print_dict_hash {
            let dict_hash = util::hash_slice(wordvec);
            util::output(&format!("{}: {:x}", config::DICT_HASH_LABEL, dict_hash));
        }

        let result = password_from_hash(hashed, salt_num, wordvec);
        util::output(&result);
    }
}

fn build_precursor(
    master_password: &str,
    site: &str,
    username: Option<&String>,
    salt_num: u8,
) -> String {
    let mut combine = String::new();

    combine.push_str(&master_password.to_lowercase());

    combine.push_str(&site.to_lowercase());

    if let Some(u) = username {
        combine.push_str(&u.to_lowercase());
    }

    combine.push_str(config::NUM_DELIM);
    combine.push_str(&salt_num.to_string());

    combine
}

fn password_from_hash(hash: util::HashResult, salt_num: u8, wordvec: &[String]) -> String {
    let mut words: [String; config::PASS_WORD_LEN] = Default::default();

    for i in 0..config::PASS_WORD_LEN {
        let v = i * 4;
        let couplet = util::combine_as_u32(hash[v], hash[v + 1], hash[v + 2], hash[v + 3]);

        let word = wordvec
            .get(util::reduce_range(couplet, wordvec.len()))
            .unwrap();
        let word_title_cased = util::as_titlecase(word);
        words[i] = String::from(word_title_cased);
    }

    let joined_words = words.join(config::WORD_DELIM);

    format!("{}{}{}", joined_words, config::NUM_DELIM, salt_num)
}

fn prompt_master_password() -> Result<String, &'static str> {
    for i in 0..config::PASS_RETRY_ATTEMPTS {
        let password = rpassword::prompt_password_stdout(config::PASS_PROMPT)
            .expect("unable to prompt stdout for password");

        let password_confirm = rpassword::prompt_password_stdout(config::PASS_CONFIRM)
            .expect("unable to prompt stdout for password");

        if password == password_confirm {
            return Ok(password);
        } else if i < config::PASS_RETRY_ATTEMPTS - 1 {
            util::output("Confirmation did not match. Try again.");
        }
    }

    Err("password mismatch failure")
}
