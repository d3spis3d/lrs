extern crate structopt;

use std::fs::{self,DirEntry};
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

fn read_dirs(p: &Path) -> Result<Vec<DirEntry>, Error> {
    let mut dir_contents = Vec::new();
    for entry in fs::read_dir(p)? {
        let e = entry?;
        dir_contents.push(e);
    }
    Ok(dir_contents)
}

struct SegmentedContents {
    files: Vec<OsString>,
    dirs: Vec<OsString>,
    symlinks: Vec<OsString>,
}

fn segment(contents: Vec<DirEntry>) -> Result<SegmentedContents, Error> {
    let segments = contents
        .into_iter()
        .fold(
            SegmentedContents { files: vec![], dirs: vec![], symlinks: vec![] },
            |mut acc, x| {
                let name = x.file_name();
                let file_type = x.file_type().unwrap();
                if file_type.is_dir() {
                    acc.dirs.push(name);
                } else if file_type.is_file() {
                    acc.files.push(name);
                } else {
                    acc.symlinks.push(name);
                }

                acc
            }
        );
    Ok(segments)
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
    let contents = read_dirs(opts.path.as_path());
    let contents = match contents {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            return ();
        }
    };

    let segmented_contents = segment(contents);
    let segmented_contents = match segmented_contents {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            return ();
        }
    };

    if opts.all {
        print_all(segmented_contents.files);
        print_all(segmented_contents.symlinks);
        print_all(segmented_contents.dirs);
        println!()
    } else {
        print(segmented_contents.files);
        print(segmented_contents.symlinks);
        print(segmented_contents.dirs);
        println!()
    }
}
