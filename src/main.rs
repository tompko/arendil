extern crate argparse;

mod epd;
mod perft;

use epd::EpdReader;
use std::fs::File;
use std::io::Read;
use std::process::exit;
use argparse::{ArgumentParser, Store};

fn main() {
    let mut perft_file_path = "".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Arendil chess engine");
        ap.refer(&mut perft_file_path)
            .add_option(&["-p", "--perft"], Store,
            "Path to perft file");
        ap.parse_args_or_exit();
    }

    if perft_file_path != "" {
        println!("{}", perft_file_path);
        let mut file = File::open(perft_file_path).unwrap();
        let mut perft_file = EpdReader::new(&mut file);
        let res = perft::test(&perft_file);

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
