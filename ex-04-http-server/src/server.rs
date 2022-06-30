// struct's resemble Class in OOP
//  but unlike Classes, they do not include functionality.
//  Instead, you add functionality to an implementation block.
pub struct Server {
    addr: String
}

// Two types of functionality within an implementation block.
//   1. http: Similar to functions in OOP
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
