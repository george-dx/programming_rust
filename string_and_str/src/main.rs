fn main() {
    let full = "bookkeeping";
    assert_eq!(&full[..4], "book");
    assert_eq!(full[..].len(), 11);
}
