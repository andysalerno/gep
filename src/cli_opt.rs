#[derive(StructOpt)]
#[structopt(name = "gep", about = "Good Enough Password generator: a simple stateless password engine")]
pub struct CliOpt {
    #[structopt(short = "s", long = "site", help = "Site name, like `hackernews` (case insensitive)")]
    pub site: String,
    #[structopt(short = "u", long = "username", help = "Username (case insensitive). Optional but recommended")]
    pub username: Option<String>,
    #[structopt(short = "d", long = "dict", help = "Path to a newline-delimited wordlist to be used instead of the included wordlist")]
    pub dict: Option<String>,
    #[structopt(short = "n", long = "num", help = "A number used to salt the hashed value")]
    pub num: Option<u8>,
    #[structopt(short = "r", long = "rand_num", help = "Pick a random u8 to salt the hash value")]
    pub rand_num: bool,
    #[structopt(short = "x", long = "hex", help = "Don't construct password from dictionary, return the raw hex instead")]
    pub hex: bool,
    #[structopt(short = "v", long = "verbose", help = "Print additional information, including the precursor hash value")]
    pub verbose: bool,
}