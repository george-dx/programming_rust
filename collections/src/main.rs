fn main() {

// Create an empty vector
let mut numbers: Vec<i32> = vec![];

// Create a vector with given contents
let words = vec!["step", "on", "no", "pets"];
// 1024 zeroed-out bytes
let mut buffer = vec![0u8; 1024];

// Convert another collection to a vector
let my_vec = words.into_iter().collect::<Vec<String>>;

// Retain trick
letm ut byte_vec = b"Missssssisssippi".to_vec();

let mut seen = HashSet::new();
by_vec.retain(|r| seen.insert(*r));

assert_eq!(&byte_vec, b"Misp");

}
