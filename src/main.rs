use clap::Parser;

const NAME: &str = "coolgi";
const AUTHOR: &str = "github.com/{lquenti,meipp}";
const VERSION: &str = "0.1";
const ABOUT: &str = "TODO"; // TODO

#[derive(Parser, Debug, Clone)]
#[clap(name = NAME, author = AUTHOR, version = VERSION, about = ABOUT, long_about = None)]
struct Cli {
    /// Mount point of the exposed subtree
    server_root: String,
    /// debug mode
    #[clap(short, long)]
    debug: bool,
}

fn main() {
    let Cli {server_root, debug} = Cli::parse();
}
