use std::fs;
use std::path::Path;
use std::io::Error;

fn read_dirs(p: &Path) -> Result<(), Error> {
    for entry in fs::read_dir(p)? {
        let e = entry?;
        println!("{:?}", e.file_name());
    }
    Ok(())
}

fn main() {
    let path = Path::new(".");
    let status = read_dirs(&path);
    match status {
        Err(e) => println!("{}", e),
        _ => (),
    }
}
