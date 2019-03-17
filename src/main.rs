extern crate structopt;

use std::path::PathBuf;
use structopt::StructOpt;

mod read;
mod printer;

#[derive(StructOpt)]
#[structopt(name = "lsr", about = "Rust implementation of UNIX ls command.")]
struct Opt {
    // Include files starting with a dot (.)
    #[structopt(short = "a")]
    all: bool,
    // The directory to list the contents of
    #[structopt(parse(from_os_str), default_value = ".")]
    path: PathBuf,
}

fn main() {
    let opts = Opt::from_args();
    let contents = read::read_dir(opts.path.as_path());
    let contents = match contents {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            return ();
        }
    };

    printer::print_contents(contents, opts.all);
}
