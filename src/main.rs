extern crate structopt;

use std::fs;
use std::ffi::OsString;
use std::path::{Path,PathBuf};
use std::io::Error;
use structopt::StructOpt;

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

fn read_dirs(p: &Path) -> Result<Vec<OsString>, Error> {
    let mut dir_contents = Vec::new();
    for entry in fs::read_dir(p)? {
        let e = entry?;
        dir_contents.push(e.file_name());
    }
    Ok(dir_contents)
}

fn print_all(contents: Vec<OsString>) {
    for c in contents {
        println!("{}", c.to_str().unwrap())
    }
}

fn print(contents: Vec<OsString>) {
    for c in contents {
        let c_string = c.to_str().unwrap();
        if c_string.chars().nth(0) != ".".chars().nth(0) {
            println!("{}", c_string)
        }
    }
}

fn main() {
    let opts = Opt::from_args();
    let contents = read_dirs(opts.path.as_path());

    let contents = match contents {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            return ();
        }
    };

    if opts.all {
        print_all(contents);
    } else {
        print(contents);
    }
}
