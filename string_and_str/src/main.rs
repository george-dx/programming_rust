use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fmt::{self, Write},
    io::BufRead,
    rc::Rc,
};

struct Complex<T> {
    re: T,
    im: T,
}

impl fmt::Display for Complex<f64> {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        let im_sign = if self.im < 0.0 { '-' } else { '+' };
        write!(dest, "{} {} {}i", self.re, im_sign, f64::abs(self.im))
    }
}

fn main() {
    let full = "bookkeeping";
    assert_eq!(&full[..4], "book");
    assert_eq!(full[..].len(), 11);

    let mut also_spaceless = "con".to_string();
    also_spaceless.extend("tri but ion".split_whitespace());
    assert_eq!(also_spaceless, "contribution");

    let mut letter = String::new();
    writeln!(letter, "Whose {} these are I think I know", "rutabagas").unwrap();
    assert_eq!(letter, "Whose rutabagas these are I think I know\n");

    let mut choco = "chocolate".to_string();
    assert_eq!(choco.drain(3..6).collect::<String>(), "col");
    assert_eq!(choco, "choate");

    let mut beverage = "a piña colada".to_string();
    beverage.replace_range(2..7, "kahula"); // ñ is two bytes
    assert_eq!(beverage, "a kahula colada");

    let haystack = "One fine day, in the middle of the night";
    assert_eq!(haystack.find(','), Some(12));
    assert_eq!(haystack.find("night"), Some(35));

    assert_eq!(
        "## Elephants".trim_start_matches(|ch: char| ch == '#' || ch.is_whitespace()),
        "Elephants"
    );

    assert!("2017".starts_with(char::is_numeric));
    let quip = "We also know there are known unknowns";
    assert_eq!(quip.find("know"), Some(8));
    assert_eq!(quip.rfind("know"), Some(31));

    assert_eq!(
        "The only thing we have to fear is fear itself".replace("fear", "spin"),
        "The only thing we have to spin is spin itself"
    );

    // replace is done eagerly
    assert_eq!("cabababababbage".replace("aba", "***"), "c***b***babbage");

    assert_eq!(
        "jimb:100:Jim Blandy:".split(':').collect::<Vec<_>>(),
        vec!["jimb", "100", "Jim Blandy", ""]
    );

    assert_eq!("\t*.rs    ".trim(), "*.rs");

    assert_eq!(format!("{}, wow", "doge"), "doge, wow");
    assert_eq!(format!("{}", true), "true");

    let good_utf8: Vec<u8> = vec![0xe9, 0x8c, 0x86];
    assert_eq!(String::from_utf8(good_utf8).ok(), Some("錆".to_string()));

    // Formatting values
    println!(
        "{:.3}µs: relocated {} at {:#x} to {:#x}, {} bytes",
        0.84391, "object", 140737488346304_usize, 6299664_usize, 64
    );

    assert_eq!(format!("{{a, c}} ⊂ {{a, b, c}}"), "{a, c} ⊂ {a, b, c}");

    let original = Rc::new("mazurka".to_string());
    let cloned = original.clone();
    let impostor = Rc::new("mazurka".to_string());
    println!("text: {}, {}, {}", original, cloned, impostor);
    println!("pointers: {:p}, {:p}, {:p}", original, cloned, impostor);

    let one_twenty = Complex {
        re: -0.5,
        im: 0.866,
    };
    assert_eq!(format!("{}", one_twenty), "-0.5 + 0.866i");

    let semver = Regex::new(r"(\d+)\.(\d+)\.(\d+)(-[-.[:alnum:]]*)?").unwrap();
    let haystack = r#"regex = "0.2.5""#;
    assert!(semver.is_match(haystack));

    lazy_static! {
        static ref SEMVER: Regex =
            Regex::new(r"(\d+)\.(\d+)\.(\d+)(-[-.[:alnum:]]*)?").expect("Error parsing regex");
    }

    let stdin = std::io::stdin();
    for line_result in stdin.lock().lines() {
        let line = line_result.unwrap();
        if let Some(match_) = SEMVER.find(&line) {
            println!("{}", match_.as_str());
        }
    }
}
