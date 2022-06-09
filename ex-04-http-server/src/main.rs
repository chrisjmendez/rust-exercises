fn main() {

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
//   method: Similar to functions in OOP
//   associated function: Similar to Static functions in OOP.
impl Server {
    // Use of "new" is simply a convention
    // Self is an alias for its type
    fn new(addr: String) -> Self {
        Self {
            // Syntactic sugar doesn't require an assignment
            addr
        }
    }
    // Run function takes ownership of the struct for this demo
    fn run(self) {

    }
}
