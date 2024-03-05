use std::collections::{binary_heap::PeekMut, BinaryHeap, HashMap, HashSet, VecDeque};

fn main() {
    // Create an empty vector
    let mut _numbers: Vec<i32> = vec![];

    // Create a vector with given contents
    let _words = vec!["step", "on", "no", "pets"];
    // 1024 zeroed-out bytes
    let mut _buffer = vec![0u8; 1024];

    // Convert another collection to a vector
    // let my_vec = words.into_iter().collect::<Vec<String>>();

    // Retain trick
    let mut byte_vec = b"Missssssisssippi".to_vec();

    let mut seen = HashSet::new();
    byte_vec.retain(|r| seen.insert(*r));

    assert_eq!(&byte_vec, b"Misp");

    // VecDequeue<T>
    let _v = VecDeque::from(vec![1, 2, 3, 4]);

    // BinaryHeap<T>
    let mut heap = BinaryHeap::from(vec![2, 3, 8, 6, 9, 5, 4]);
    assert_eq!(heap.peek(), Some(&9));
    assert_eq!(heap.pop(), Some(9));

    // HashMap<K,V> and BTreeMap<K,V>
    let ballots = vec![String::from("abcd"), String::from("efgh")];
    let mut vote_counts: HashMap<String, usize> = HashMap::new();
    for name in ballots {
        let count = vote_counts.entry(name).or_insert(0);
        *count += 1;
    }

    println!("{:?}", vote_counts);

    let text = "this is a random this text this";

    let mut word_frequency: HashMap<&str, u32> = HashMap::new();
    for c in text.split_whitespace() {
        word_frequency
            .entry(c)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    println!("{:?}", word_frequency);
}
