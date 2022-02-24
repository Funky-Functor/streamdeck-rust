mod cmd_line_args;

use crate::cmd_line_args::get_arguments;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(websocket_args) = get_arguments(&args) {
        println!("Value is {:?}", websocket_args);
    } else {
        println!("getArguments couldn't find a value.")
    }
}
