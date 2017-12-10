pub const PASS_PROMPT: &'static str = "Master password: ";
pub const PASS_CONFIRM: &'static str = "Confirm master password: ";
pub const EXCEEDED_ATTEMPTS: &'static str = "Exceeded password attempts. Quitting.";
pub const DEBUG_WARNING: &'static str =
    "WARNING: Debug mode enabled; this will output your raw master password to stdout!";
pub const DICT_HASH_LABEL: &'static str = "Wordlist SHA-3";

pub const DEFAULT_DICT_PATH: &'static str = "dictionary.txt";
pub const NUM_DELIM: &'static str = ":";
pub const WORD_DELIM: &'static str = "";
pub const PASS_WORD_LEN: usize = 4;
pub const PASS_RETRY_ATTEMPTS: u8 = 3;
