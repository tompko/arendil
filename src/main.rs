extern crate clap;

mod epd;
mod perft;

use epd::EpdReader;
use std::fs::File;
use std::process::exit;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Arendil")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Chris T <tompko@gmail.com>")
        .about("A chess engine in Rust")
        .arg(Arg::with_name("perft")
             .short("p")
             .long("perft")
             .value_name("FILE")
             .help("Runs a perft test with the file")
             .takes_value(true))
        .get_matches();

    let perft = matches.value_of("perft");
    if perft.is_some() {
        let perft = perft.unwrap();
        println!("{}", perft);
        let mut file = File::open(perft).unwrap();
        let mut perft_file = EpdReader::new(&mut file);
        let res = perft::test(&mut perft_file);

        match res {
            Ok(_) => {
                println!("Perft pass");
                return;
            },
            Err(_) => {

                println!("Perft fail");
                exit(-1);
            }
        }
    }
}
