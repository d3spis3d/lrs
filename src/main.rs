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

fn segment(contents: Result<Vec<DirEntry>, Error>) -> Result<SegmentedContents, Error> {
    let contents = contents?;
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

fn sort_segments(segments: Result<SegmentedContents, Error>) -> Result<SegmentedContents, Error> {
    let mut segments = segments?;
    segments.files.sort_unstable();
    segments.dirs.sort_unstable();
    segments.symlinks.sort_unstable();

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
    let segmented_contents = segment(contents);
    let sorted_contents = sort_segments(segmented_contents);
    let sorted_contents = match sorted_contents {
        Ok(c) => c,
        Err(e) => {
            println!("{}", e);
            return ();
        }
    };

    if opts.all {
        print_all(sorted_contents.files);
        print_all(sorted_contents.symlinks);
        print_all(sorted_contents.dirs);
        println!()
    } else {
        print(sorted_contents.files);
        print(sorted_contents.symlinks);
        print(sorted_contents.dirs);
        println!()
    }
}
