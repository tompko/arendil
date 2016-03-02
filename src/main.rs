extern crate argparse;

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
    }
}
