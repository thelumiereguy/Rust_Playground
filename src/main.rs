use std::str;

use crate::variables::vars;

mod printing;
mod variables;
mod types;
mod strings;
mod tuples;
mod arrays;
mod vectors;
mod conditionals;
mod loops;
mod functions;
mod pointers;
mod structs;
mod enums;
mod cli;


fn main() {
    // printing::print::run();
    // vars::run();
    cli::cli::run();
}
