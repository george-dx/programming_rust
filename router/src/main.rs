use rand::random;
use std::collections::HashMap;
use std::iter::from_fn;
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
}
