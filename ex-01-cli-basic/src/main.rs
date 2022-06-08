#![allow(unused)]
// Clap Command is the main macro
// Example: https://docs.rs/clap/3.0.0-beta.5/clap/index.html#features
use clap::{Arg, Command};

fn main() {

    let matches = Command::new("My CLI Command")
        .version("v1.0" )
        .about( "Description" )
        .author( "Chris. <support@chrisaiv.com>")
        .override_help("My First Command" )
        .arg(Arg::new("config")
            .short('c')
            .long("config")
            .value_name("FILE")
            .help("-c Sets a custom config file")
            .takes_value(true))
        .arg(Arg::new("INPUT")
            .help("Sets the input file to use")
            .required(true)
            .index(1))
        .arg(Arg::new("v")
            .short('v')
            .long("verbose")
            .multiple_occurrences(true)
            .takes_value(true)
            .help("-v [0,1,2,-] Sets the level of verbosity"))
        .subcommand(Command::new("test")
            .override_help("controls testing features")
            .version("1.3")
            .author("Someone E. <someone_else@other.com>")
            .arg(Arg::new("debug")
                .short('d')
                .help("print debug information verbosely")))
        .get_matches();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(i) = matches.value_of("INPUT") {
        println!("Value for input: {}", i);
    }

    if let Some(c) = matches.value_of("config") {
        println!("Value for config: {}", c);
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match matches.occurrences_of("v") {
        0 => println!("Verbose mode is off"),
        1 => println!("Verbose mode is kind of on"),
        2 => println!("Verbose mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level Command
    if let Some(ref matches) = matches.subcommand_matches("test") {
        // "$ myCommand test" was run
        if matches.is_present("debug") {
            // "$ myCommand test -d" was run
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
    }

}
