extern crate structopt;
use structopt::StructOpt;

// A. Annotate the struct with a custom derive attribute.
// This allows you to take the struct as the arg definition
//    and generate the argument parsers accordingly.
#[derive(StructOpt)]
struct Options {
    message: String
}

fn main() {
    // B. Parse the arguments and fills them into Options struct
    let options = Options::from_args();
    // C. As a struct, you can access the individual fields
    let message = options.message;

    println!("{}",message);
}
