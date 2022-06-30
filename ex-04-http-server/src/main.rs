use http::request::Request;
use server::Server;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
        server.run();
}

mod server {
    // struct's resemble Class in OOP
    //  but unlike Classes, they do not include functionality.
    //  Instead, you add functionality to an implementation block.
    pub struct Server {
        addr: String
    }

    // Two types of functionality within an implementation block.
    //   1. method: Similar to functions in OOP
    //   2. associated function: Similar to Static functions in OOP.
    impl Server {
        // A. Use of "new" is simply a convention
        // B. Self is an alias for its type so "Self" and "Server" are interchangable
        pub fn new(addr: String) -> Self {
            // C. Syntactic sugar doesn't require an assignment addr:̶a̶d̶d̶r̶
            Server { addr }
        }

        // D. Run function takes ownership of the struct for this demo
        pub fn run(self) {
            println!("Listening on {}", self.addr)
        }
    }
}

// HTTP Module
mod http {
    pub mod request {
        use super::method::Method;
        // Rust enums are similar to the algabraic data-types in Haskell
        pub struct Request {
            path: String,
            query_string: Option<String>, //Option functions like Null in a type-safe way
            method: Method,
        }
    }

    pub mod method {
        pub enum Method {
            GET,
            DELETE,
            POST,
            PUT,
            CONNECT,
            OPTIONS,
            TRACE,
            PATCH
        }
    }
}
