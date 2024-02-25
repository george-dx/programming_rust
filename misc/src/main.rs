use std::error::Error;
use std::fs;
use std::io::{self, stderr, Result, Write};
use std::ops::{Add, AddAssign, Mul, Neg};
use std::path::Path;

// /// A trait for cahracters, items and scenery - anything in
// /// the game world that's visible on scree
// trait Visible {
//     /// Render this object on the given canvas
//     fn draw(&self, canvas: &mut Canvas);

//     /// Return true if clicking at (x, y) should
//     /// select this object
//     fn hit_test(&self, x: i32, y: i32) -> bool;
// }

// impl Visible for Broom {
//     fn draw(&self, canvas: &mut Canvas) {
//         for y in self.broomstick_range() {
//             canvas.write_at(self.x, y, '|');
//         }
//         canvas.write_at(self.x, self.y, 'M');
//     }

//     fn hit_test(&self, x: i32, y: i32) -> bool {
//         self.x == x && self.y - self.height - 1 <= y && y <= self.y
//     }
// }

// impl Broom {
//     /// Helper function used by Broom::draw() below.
//     fn broomstick_range(&self) -> Range<i32> {
//         self.y - self.height - 1..self.y
//     }
// }

// type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
// type GenericResult<T> = Result<T, GenericError>;

struct Sink;

impl Write for Sink {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        // Claim to have successfully written the whole buffer.
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

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

// fn _read_numbers(file: &mut dyn BufRead) -> GenericResult<Vec<i64>> {
//     let mut numbers = vec![];
//     for line_result in file.lines() {
//         let line = line_result?; // reading lines can fail
//         numbers.push(line.parse()?); // parsing integers can fail
//     }
//     Ok(numbers)
// }

trait Spliceable {
    fn splice(&self, other: &Self) -> Self;
}

trait MicroSpliceable: Spliceable {
    fn microsplice(&self, other: &Self) -> Self;
}

struct CherryTree {
    size: i32,
}
struct MelonTree {
    size: i32,
}

struct CoconutTree {
    size: i32,
}

impl Spliceable for CoconutTree {
    fn splice(&self, other: &Self) -> Self {
        CoconutTree {
            size: other.size + self.size,
        }
    }
}

impl MicroSpliceable for CoconutTree {
    fn microsplice(&self, other: &Self) -> Self {
        CoconutTree {
            size: other.size / 10 + self.size,
        }
    }
}

impl Spliceable for CherryTree {
    fn splice(&self, other: &Self) -> Self {
        CherryTree {
            size: other.size + self.size,
        }
    }
}

impl Spliceable for MelonTree {
    fn splice(&self, other: &Self) -> Self {
        MelonTree {
            size: other.size + self.size,
        }
    }
}

fn dot<N>(v1: &[N], v2: &[N]) -> N
where
    N: Add<Output = N> + Mul<Output = N> + Default + Copy,
{
    let mut total = N::default();
    for i in 0..v1.len() {
        total = total + v1[i] * v2[i];
    }
    total
}

#[test]
fn test_dot() {
    assert_eq!(dot(&[1, 2, 3, 4], &[1, 1, 1, 1]), 10);
    assert_eq!(dot(&[53.0, 7.0], &[1.0, 5.0]), 88.0);
}

#[derive(Clone, Copy, Debug)]
struct Complex<T> {
    /// Real portion of the complex number
    re: T,
    /// Imaginary portion of the complex number
    im: T,
}

impl<T> Neg for Complex<T>
where
    T: Neg<Output = T>,
{
    type Output = Complex<T>;
    fn neg(self) -> Complex<T> {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl<T> AddAssign for Complex<T>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Complex<T>) {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

struct Appellation {
    name: String,
    nicknames: Vec<String>,
}

impl Drop for Appellation {
    fn drop(&mut self) {
        print!("Dropping {}", self.name);
        if !self.nicknames.is_empty() {
            print!(" (AKA {})", self.nicknames.join(", "));
        }
        println!("");
    }
}

fn main() -> Result<()> {
    let mut out = Sink;
    out.write_all(b"hello world\n")?;

    let mut a = Appellation {
        name: "Zeus".to_string(),
        nicknames: vec![
            "cloud collector".to_string(),
            "king of the gods".to_string(),
        ],
    };
    println!("before assignment");
    a = Appellation {
        name: "Hera".to_string(),
        nicknames: vec![],
    };
    println!("at end of block");

    Ok(())
}
