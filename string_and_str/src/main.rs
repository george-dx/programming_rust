use std::fmt::Write;

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
}
