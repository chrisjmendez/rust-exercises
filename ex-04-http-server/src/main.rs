
fn main() {
    let get    = Method::GET;
    let delete = Method::DELETE;
    let post   = Method::POST;
    let put    = Method::PUT;

    let server = Server::new("127.0.0.1:8080".to_string());
        server.run();
}

// struct's resemble Class in OOP
//   but unlike Classes, they do not include functionality.
//   Instead, you add functionality to an implementation block.
struct Server {
    addr: String
}

// Two types of functionality within an implementation block.
//   1. method: Similar to functions in OOP
//   2. associated function: Similar to Static functions in OOP.
impl Server {
    // A. Use of "new" is simply a convention
    // B. Self is an alias for its type so "Self" and "Server" are interchangable
    fn new(addr: String) -> Self {
        Server {
            // C. Syntactic sugar doesn't require an assignment addr:̶a̶d̶d̶r̶
            addr
        }
    }

    // D. Run function takes ownership of the struct for this demo
    fn run(self) {
        println!("Listening on {}", self.addr)
    }
}

// Rust enums are similar to the algabraic data-types in Haskell
struct Request {
    path: String,
    // This is how, in Rust, you can introduce Null in a type-safe way
    query_string: Option<String>,
    method: Method
}

enum Method {
    GET,
    DELETE,
    POST,
    PUT,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}
