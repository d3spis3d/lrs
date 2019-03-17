extern crate structopt;

use std::ffi::OsString;
use std::path::PathBuf;
use structopt::StructOpt;

mod read;

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

fn print_all(contents: Vec<OsString>) {
    for c in contents {
        print!("{} ", c.to_str().unwrap())
    }
}

fn print(contents: Vec<OsString>) {
    for c in contents {
        let c_string = c.to_str().unwrap();
        if c_string.chars().nth(0) != ".".chars().nth(0) {
            print!("{} ", c_string)
        }
    }
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

    if opts.all {
        print_all(contents.files);
        print_all(contents.symlinks);
        print_all(contents.dirs);
        println!()
    } else {
        print(contents.files);
        print(contents.symlinks);
        print(contents.dirs);
        println!()
    }
}
