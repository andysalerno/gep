extern crate rpassword;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use cli_opt::CliOpt;
use dict_reader::DictReader;
use password_engine::PasswordEngine;
use structopt::StructOpt;

mod cli_opt;
mod config;
mod dict_reader;
mod password_engine;
mod util;

fn main() {
    let opt = CliOpt::from_args();

    let info_dest = util::WriteDest::StdOut;
    let password_dest = match opt.output_filename {
        Some(s) => util::WriteDest::Filename(s),
        None => util::WriteDest::StdOut,
    };

    if opt.verbose {
        util::write(&info_dest, config::VERBOSE_WARNING);
    }

    let master_password = match opt.password {
        Some(p) => p,
        None => prompt_master_password()
            .unwrap_or_else(|_| util::exit_with_message((config::EXCEEDED_ATTEMPTS))),
    };

    let salt_num = if opt.rand_num {
        Some(util::secure_rand_u8())
    } else {
        opt.salt_num
    };

    let dict_reader = match opt.dict {
        Some(dict_path) => DictReader::new(&dict_path),
        None => DictReader::new(config::DEFAULT_DICT_PATH),
    };

    let wordvec = dict_reader.get_wordvec();

    check_dict_len(&wordvec);

    let generated_password = PasswordEngine::generate(
        &opt.site,
        opt.username.as_ref().map(String::as_str),
        &master_password,
        &wordvec,
        salt_num,
    );

    if opt.print_dict_hash {
        let dict_hash = util::hash_slice(&wordvec);
        util::write(
            &info_dest,
            &format!("{}: {:x}\n", config::LABEL_DICT_HASH, dict_hash),
        );
    }

    if opt.verbose {
        util::write(
            &info_dest,
            &format!(
                "{}: {}\n",
                config::LABEL_PRECURSOR,
                generated_password.precursor()
            ),
        );
    }

    if opt.hex_only {
        util::write(
            &password_dest,
            &format!("{:x}\n", generated_password.precursor_hashed()),
        );
    } else {
        util::write(
            &password_dest,
            &format_password_display(generated_password.password()),
        );
    }
}

fn prompt_master_password() -> Result<String, &'static str> {
    for i in 0..config::MASTER_RETRY_ATTEMPTS {
        let password = rpassword::prompt_password_stdout(config::PASS_PROMPT)
            .expect(config::ERR_STDOUT_PROMPT);

        let password_confirm = rpassword::prompt_password_stdout(config::PASS_CONFIRM)
            .expect(config::ERR_STDOUT_PROMPT);

        if password == password_confirm {
            return Ok(password);
        } else if i < config::MASTER_RETRY_ATTEMPTS - 1 {
            util::write(&util::WriteDest::StdOut, config::PASS_CONFIRM_MISMATCH);
        }
    }

    Err("password mismatch failure")
}

fn check_dict_len(dict: &[String]) {
    if dict.len() < config::MIN_DICT_SIZE {
        util::write(&util::WriteDest::StdOut, &config::small_dict_warning());
    }
}

fn format_password_display(password: &str) -> String {
    format!("{}\n", password)
}
