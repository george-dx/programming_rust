use rand::random;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::prelude::*;
use std::iter::{from_fn, once, repeat};
use std::str::FromStr;

struct Request {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

struct Response {
    code: u32,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

type BoxedCallback = Box<dyn Fn(&Request) -> Response>;

struct BasicRouter {
    routes: HashMap<String, BoxedCallback>,
}

impl BasicRouter {
    /// Create an empty router.
    fn new() -> BasicRouter {
        BasicRouter {
            routes: HashMap::new(),
        }
    }

    /// Add a route to the router.
    fn add_route<C>(&mut self, url: &str, callback: C)
    where
        C: Fn(&Request) -> Response + 'static,
    {
        self.routes.insert(url.to_string(), Box::new(callback));
    }

    // fn handle_request(&self, request: &Request) -> Response {
    //     match self.routes.get(&request.url) {
    //         None => not_found_response(),
    //         Some(callback) => callback(request),
    //     }
    // }
}

struct FnPointerRouter {
    routes: HashMap<String, fn(&Request) -> Response>,
}

impl FnPointerRouter {
    fn new() -> FnPointerRouter {
        FnPointerRouter {
            routes: HashMap::new(),
        }
    }

    fn add_route(&mut self, url: &str, callback: fn(&Request) -> Response) {
        self.routes.insert(url.to_string(), callback);
    }
}

struct Order {
    items: Vec<OrderItem>,
}

struct OrderItem {
    product_id: u32,
    quantity: u32,
    price: f64,
}

impl IntoIterator for Order {
    type Item = OrderItem;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

fn cmp(lhs: &f64, rhs: &f64) -> Ordering {
    lhs.partial_cmp(rhs).unwrap()
}

fn main() {
    let lengths: Vec<f64> = from_fn(|| Some((random::<f64>() - random::<f64>()).abs()))
        .take(10)
        .collect();
    println!("{:?}", lengths);

    let text = " ponies \n giraffes\niguanas \nsquid".to_string();
    let v: Vec<&str> = text
        .lines()
        .map(str::trim)
        .filter(|s| *s != "iguanas")
        .collect();
    assert_eq!(v, ["ponies", "giraffes", "squid"]);

    let text = "1\nfrond .25 289\n3.1415 estuary\n";
    for number in text
        .split_whitespace()
        .filter_map(|w| f64::from_str(w).ok())
    {
        println!("{:4.2}", number.sqrt());
    }

    let mut major_cities = HashMap::new();
    major_cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
    major_cities.insert("The United States", vec!["Portland", "Nashville"]);
    major_cities.insert("Brazil", vec!["São Paulo", "Brasília"]);
    major_cities.insert("Kenya", vec!["Nairobi", "Mombasa"]);
    major_cities.insert("The Netherlands", vec!["Amsterdam", "Utrecht"]);
    let countries = ["Japan", "Brazil", "Kenya"];
    for &city in countries.iter().flat_map(|country| &major_cities[country]) {
        println!("{}", city);
    }

    // Reversible iterators and rev
    let bee_parts = ["head", "thorax", "abdomen"];
    let mut iter = bee_parts.iter();

    assert_eq!(iter.next(), Some(&"head"));
    assert_eq!(iter.next_back(), Some(&"abdomen"));
    assert_eq!(iter.next(), Some(&"thorax"));
    assert_eq!(iter.next_back(), None);
    assert_eq!(iter.next(), None);

    let meals = ["breakfast", "lunch", "dinner"];
    let mut rev_iter = meals.iter().rev();
    assert_eq!(rev_iter.next(), Some(&"dinner"));
    assert_eq!(rev_iter.next(), Some(&"lunch"));
    assert_eq!(rev_iter.next(), Some(&"breakfast"));
    assert_eq!(rev_iter.next(), None);

    // Chain
    let v: Vec<i32> = (1..4).chain(vec![20, 30, 40]).collect();
    assert_eq!(v, [1, 2, 3, 20, 30, 40]);

    let r_v: Vec<i32> = (1..4).chain(vec![20, 30, 40]).rev().collect();
    assert_eq!(r_v, [40, 30, 20, 3, 2, 1]);

    // Enumerate - produces pairs (0, A), (1, B), (2, C)

    // Zip
    let v: Vec<_> = (0..).zip("ABCD".chars()).collect();
    assert_eq!(v, vec![(0, 'A'), (1, 'B'), (2, 'C'), (3, 'D')]);

    // By_ref
    let message = "To: jimb\r\n\
    From: id\r\n\
    \r\n\
    Oooooh, donuts!!\r\n";

    let mut lines = message.lines();
    println!("Headers:");
    for header in lines.by_ref().take_while(|l| !l.is_empty()) {
        println!("{}", header);
    }

    println!("\nBody:");
    for body in lines {
        println!("{}", body);
    }

    // Cloned, copied
    let a = ['1', '2', '3', '∞'];
    assert_eq!(a.iter().next(), Some(&'1'));
    assert_eq!(a.iter().cloned().next(), Some('1'));

    // Cycle
    let fizzes = repeat("").take(2).chain(once("fizz")).cycle();
    let buzzes = repeat("").take(4).chain(once("buzz")).cycle();
    let fizzes_buzzes = fizzes.zip(buzzes);
    let fizz_buzz = (1..100).zip(fizzes_buzzes).map(|tuple| match tuple {
        (i, ("", "")) => i.to_string(),
        (_, (fizz, buzz)) => format!("{}{}", fizz, buzz),
    });
    for line in fizz_buzz {
        println!("{}", line);
    }

    // Consuming iterators - count, sum, product
    // let stdin = std::io::stdin();
    // println!("{}", stdin.lock().lines().count());
    println!("{}", (1..=10).sum::<i32>());

    // max, min
    assert_eq!([-2, 0, 1, 0, -2, -5].iter().max(), Some(&1));

    // max_by, min_by
    let numbers = [1.0, 4.0, 2.0];
    assert_eq!(numbers.iter().copied().max_by(cmp), Some(4.0));

    // let numbers = [1.0, 4.0, std::f64::NAN, 2.0];
    // assert_eq!(numbers.iter().copied().max_by(cmp), Some(4.0)); // panics

    // Comparing Item Sequences - any, all
    let id = "Iterator";

    assert!(id.chars().any(char::is_uppercase));
    assert!(!id.chars().all(char::is_uppercase));

    // position, rposition, and ExactSizeIterator
    let text = "Xerxes";
    assert_eq!(text.chars().position(|c| c == 'e'), Some(1));
    assert_eq!(text.chars().position(|c| c == 'z'), None);

    // fold and rfold
    let a = [5, 6, 7, 8, 9, 10];

    assert_eq!(a.iter().fold(0, |n, _| n + 1), 6); // count
    assert_eq!(a.iter().fold(0, |n, i| n + i), 45); // sum

    assert_eq!(a.iter().cloned().fold(i32::min_value(), std::cmp::max), 10); // max

    // try_fold - same as fold, returning Result

    // nth, nth_back
    let mut squares = (0..10).map(|i| i * i);

    assert_eq!(squares.nth(4), Some(16));
    assert_eq!(squares.nth(6), None);

    // last
    let squares = (0..11).map(|i| i * i);
    assert_eq!(squares.last(), Some(100));

    // find, rfind, find_map
    let mut populations = HashMap::new();
    populations.insert("Portland", 583_776);
    populations.insert("Fossil", 449);
    populations.insert("Greenhorn", 2);
    populations.insert("Boring", 7_762);
    populations.insert("The Dalles", 15_340);

    assert_eq!(
        populations.iter().find(|&(_name, &pop)| pop > 1_000_000),
        None
    );
    assert_eq!(
        populations.iter().find(|&(_name, &pop)| pop > 500_000),
        Some((&"Portland", &583_776))
    );
}
