use std::path::Path;
use std::fs::{self,DirEntry};
use std::io::Error;
use std::ffi::OsString;

pub struct SegmentedContents {
    pub files: Vec<OsString>,
    pub dirs: Vec<OsString>,
    pub symlinks: Vec<OsString>,
}

pub fn read_dir(p: &Path) -> Result<SegmentedContents, Error> {
    let contents = read_dir_contents(p);
    let segmented_contents = segment(contents);
    sort_segments(segmented_contents)
}

fn read_dir_contents(p: &Path) -> Result<Vec<DirEntry>, Error> {
    let mut dir_contents = Vec::new();
    for entry in fs::read_dir(p)? {
        let e = entry?;
        dir_contents.push(e);
    }
    Ok(dir_contents)
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
