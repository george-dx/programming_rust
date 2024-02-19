use std::error::Error;
use std::fs;
use std::io::{self, stderr, BufRead, Write};
use std::path::Path;

/// A trait for cahracters, items and scenery - anything in
/// the game world that's visible on scree
trait Visible {
    /// Render this object on the given canvas
    fn draw(&self, canvas: &mut Canvas);

    /// Return true if clicking at (x, y) should
    /// select this object
    fn hit_test(&self, x: i32, y: i32) -> bool;
}

impl Visible for Broom {
    fn draw(&self, canvas: &mut Canvas) {
        for y in self.y - self.height - 1..self.y {
            canvas.write_at(self.x, y, '|');
        }
        canvas.write_at(self.x, self.y, 'M');
    }

    fn hit_test(&self, x: i32, y: i32) -> bool {
        self.x == x && self.y - self.height - 1 <= y && y <= self.y
    }
}

type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
type GenericResult<T> = Result<T, GenericError>;

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

fn _read_numbers(file: &mut dyn BufRead) -> GenericResult<Vec<i64>> {
    let mut numbers = vec![];
    for line_result in file.lines() {
        let line = line_result?; // reading lines can fail
        numbers.push(line.parse()?); // parsing integers can fail
    }
    Ok(numbers)
}

fn main() {}
