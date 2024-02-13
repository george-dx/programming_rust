use std::error::Error;
use std::fs;
use std::io;
use std::io::{stderr, Write};
use std::path::Path;

///Dump an error message to `stderr`.
///
/// If another error happens while building the error message
/// or writing to `stderr`, it is ignored.

fn _print_error(mut err: &dyn Error) {
    let _ = writeln!(stderr(), "error: {}", err);
    while let Some(source) = err.source() {
        let _ = writeln!(stderr(), "caused by: {}", source);
        err = source;
    }
}

fn _move_all(src: &Path, dst: &Path) -> io::Result<()> {
    for entry_result in src.read_dir()? {
        //opening dir could fail
        let entry = entry_result?; // reading dir could fail
        let dst_file = dst.join(entry.file_name());
        fs::rename(entry.path(), dst_file)?; // renaming could fain
    }
    Ok(())
}

fn main() {}
