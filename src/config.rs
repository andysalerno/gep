pub const PASS_PROMPT: &'static str = "Master password: ";
pub const PASS_CONFIRM: &'static str = "Confirm master password: ";
pub const PASS_CONFIRM_MISMATCH: &'static str = "Confirmation did not match. Try again.";
pub const EXCEEDED_ATTEMPTS: &'static str = "Exceeded password attempts. Quitting.";
pub const DEBUG_WARNING: &'static str =
    "WARNING: Debug mode enabled; this will output your raw master password to stdout!";
pub const DICT_HASH_LABEL: &'static str = "Wordlist SHA-3";

pub const TAG_PRECURSOR: &'static str = "Precursor";

pub const STDOUT_PROMPT_ERR: &'static str = "Unable to prompt stdout for master password.";

pub const PASS_OUTPUT_FRAME: &'static str = "////////////////";

pub const MIN_ALLOWED_SIZE: usize = 10_000;
pub fn small_dict_warning() -> String {
    format!(
        "WARNING: dictionary size is below the recommended limit of {}.",
        MIN_ALLOWED_SIZE
    )
}

pub const WORDLIST_READ_ERR: &'static str = "Unable to open wordlist file with path";

pub const DEFAULT_DICT_PATH: &'static str = "dictionary.txt";
pub const NUM_DELIM: &'static str = ":";
pub const WORD_DELIM: &'static str = "";
pub const PASS_WORD_LEN: usize = 4;
pub const PASS_RETRY_ATTEMPTS: u8 = 3;
