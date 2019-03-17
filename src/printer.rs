use std::ffi::OsString;
use crate::read::SegmentedContents;

fn print_all(contents: Vec<OsString>) {
    for c in contents {
        print!("{} ", c.to_str().unwrap())
    }
}

fn print_without_hidden(contents: Vec<OsString>) {
    for c in contents {
        let c_string = c.to_str().unwrap();
        if c_string.chars().nth(0) != ".".chars().nth(0) {
            print!("{} ", c_string)
        }
    }
}

pub fn print_contents(contents: SegmentedContents, all: bool) {
    if all {
        print_all(contents.files);
        print_all(contents.symlinks);
        print_all(contents.dirs);
        println!()
    } else {
        print_without_hidden(contents.files);
        print_without_hidden(contents.symlinks);
        print_without_hidden(contents.dirs);
        println!()
    }
}
