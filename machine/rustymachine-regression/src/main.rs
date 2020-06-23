extern crate serde;

#[macro_use]
extern crate serde_derive;

use std::env::args;
use std::process::exit;
use std::vec::Vec;

mod lin_reg;

fn main() {
    let args: Vec<String> = args().collect();
    let model = if args.len() < 2 {
        None
    } else {
        Some(args[1].as_str())
    };
    let res = match model {
        None => {
            println!("nothing",);
            Ok(())
        }
        Some("lr") => lin_reg::run(),
        Some(_) => lin_reg::run(),
    };
    exit(match res {
        Ok(_) => 0,
        Err(e) => {
            println!("{}", e);
            1
        }
    })
}
