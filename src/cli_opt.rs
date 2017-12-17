#[derive(StructOpt)]
#[structopt(name = "gep",
            about = "Good Enough Password generator: a simple stateless password engine")]
pub struct CliOpt {
    #[structopt(short = "s", long = "site", help = "Site name, like `hackernews` (case insensitive)")]
    pub site: String,
    #[structopt(short = "u", long = "username", help = "Username (case insensitive)")]
    pub username: Option<String>,
    #[structopt(short = "p", long = "password", help = "If provided, skips password prompt")]
    pub password: Option<String>,
    #[structopt(short = "d", long = "dict", help = "Path to a newline-delimited dictionary to be used instead of the included dictionary")]
    pub dict: Option<String>,
    #[structopt(short = "n", long = "num", help = "A number used to salt the hashed value")]
    pub salt_num: Option<u8>,
    #[structopt(short = "r", long = "rand-num", help = "Pick a random u8 to salt the hash value")]
    pub rand_num: bool,
    #[structopt(short = "x", long = "hex", help = "Return the raw hash hex output, instead of building a password from dictionary words.")]
    pub hex_only: bool,
    #[structopt(short = "D", long = "debug", help = "Print additional information, including the precursor hash value. WARNING: this includes the master password, which is also printed!")]
    pub debug: bool,
    #[structopt(short = "H", long = "dict-hash", help = "Print the dictionary hash.")]
    pub print_dict_hash: bool,
    #[structopt(short = "o", long = "output-file", help = "Path to output file where password is printed; defaults to stdout")]
    pub output_filename: Option<String>,
}
