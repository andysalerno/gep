pub const PASS_PROMPT: &'static str = "Master password: ";
pub const PASS_CONFIRM: &'static str = "Confirm master password: ";

pub const WORDLIST_READ_ERR: &'static str = "Unable to open wordlist file with path";
pub const STDOUT_PROMPT_ERR: &'static str = "Unable to prompt stdout for master password.";
pub const PASS_CONFIRM_MISMATCH: &'static str = "Confirmation did not match. Try again.\n";
pub const EXCEEDED_ATTEMPTS: &'static str = "Exceeded password attempts. Quitting.\n";
pub const DEBUG_WARNING: &'static str =
    "WARNING: Debug mode enabled; this will output your raw master password to stdout!\n";

pub const LABEL_DICT_HASH: &'static str = "Wordlist SHA-3";
pub const LABEL_PRECURSOR: &'static str = "Precursor";

pub const DEFAULT_DICT_PATH: &'static str = "dictionary.txt";
pub const MIN_DICT_SIZE: usize = 10_000;
pub const SALT_DELIM: &'static str = ":";
pub const WORD_DELIM: &'static str = "";
pub const PASS_WORD_COUNT: usize = 4;
pub const MASTER_RETRY_ATTEMPTS: u8 = 3;

pub fn small_dict_warning() -> String {
    format!(
        "WARNING: dictionary size is below the recommended limit of {}.",
        MIN_DICT_SIZE
    )
}
