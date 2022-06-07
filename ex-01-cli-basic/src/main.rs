// Clap app is the main macro
// Example: https://docs.rs/clap/3.0.0-beta.5/clap/macro.clap_app.html#examples

use clap::{clap_app, crate_version};

fn main() {
    let clap = clap_app!( app =>
        (version:crate_version!())
        (author:"Chris Mendez")
        (about:"My First Exercise!")
        (@arg input: +required "Sets the input file")
    )
    .get_matches();

    println!("Input = {:?}", clap.value_of("input"));

    println!("Complete");
}
