use config;
use util;
use util::HashResult;

pub struct GeneratedPassword {
    gen_password: String,
    precursor: String,
    precursor_hashed: HashResult,
}

impl GeneratedPassword {
    pub fn password(&self) -> &str {
        &self.gen_password
    }

    pub fn precursor(&self) -> &str {
        &self.precursor
    }

    pub fn precursor_hashed(&self) -> &HashResult {
        &self.precursor_hashed
    }
}

pub struct PasswordEngine {}

impl PasswordEngine {
    pub fn generate(
        site: &str,
        username: Option<&str>,
        master_password: &str,
        wordlist: &[String],
        salt_opt: Option<u8>,
    ) -> GeneratedPassword {
        // If not provided, the salt number is generated
        // from the hash of the master password
        let salt_num = if let Some(n) = salt_opt {
            n
        } else {
            // TODO: should it be the sum of every byte?
            let master_hashed = util::hash(master_password);
            let collected_byte = master_hashed[0];

            collected_byte as u8
        };

        let precursor = PasswordEngine::build_precursor(master_password, site, username, salt_num);

        let precursor_hashed = util::hash(&precursor);

        let password = PasswordEngine::password_from_hash(precursor_hashed, salt_num, wordlist);

        GeneratedPassword {
            precursor: precursor,
            gen_password: password,
            precursor_hashed,
        }
    }

    fn build_precursor(
        master_password: &str,
        site: &str,
        username: Option<&str>,
        salt_num: u8,
    ) -> String {
        let mut combine = String::new();

        combine.push_str(&master_password.to_lowercase());

        combine.push_str(&site.to_lowercase());

        if let Some(u) = username {
            combine.push_str(&u.to_lowercase());
        }

        combine.push_str(config::SALT_DELIM);
        combine.push_str(&salt_num.to_string());

        combine
    }

    fn password_from_hash(hash: util::HashResult, salt_num: u8, wordvec: &[String]) -> String {
        let mut words: [String; config::PASS_WORD_COUNT] = Default::default();

        for i in 0..config::PASS_WORD_COUNT {
            let v = i * 4;
            let couplet = util::combine_as_u32(hash[v], hash[v + 1], hash[v + 2], hash[v + 3]);

            let word = wordvec
                .get(util::reduce_range(couplet, wordvec.len()))
                .unwrap();
            let word_title_cased = util::as_titlecase(word);
            words[i] = String::from(word_title_cased);
        }

        let joined_words = words.join(config::WORD_DELIM);

        format!("{}{}{}", joined_words, config::SALT_DELIM, salt_num)
    }
}
