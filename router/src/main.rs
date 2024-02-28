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

struct BasicRouter<C>
where
    C: Fn(&Request) -> Response,
{
    routes: HashMap<String, C>,
}

impl<C> BasicRouter<C>
where
    C: Fn(&Request) -> Response,
{
    /// Create an empty router.
    fn new() -> BasicRouter<C> {
        BasicRouter {
            routes: HashMap::new(),
        }
    }

    /// Add a route to the router.
    fn add_route(&mut self, url: &str, callback: C) {
        self.routes.insert(url.to_string(), callback);
    }
}

fn main() {
    println!("Hello, world!");
}
