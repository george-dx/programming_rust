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
}
