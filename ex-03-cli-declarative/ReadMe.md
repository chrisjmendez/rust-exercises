# Declarative-style Command Line App

# Context

Custom derive is a feature in Rust that automatically 
generates a default implementation of a trait by annotating 
a struct. You can define a struct containing the arguments 
you want and annotate it with #[derive(StructOpt)]. A macro 
defined by the StructOpt automatically implements the StructOpt 
trait for the struct. This implementation will contain the 
necessary clap code for parsing the arguments.

# Getting Started

```
 cargo run -- "hello world"
```
