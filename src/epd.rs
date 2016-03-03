use std::io::Read;
use std::path::Path;

pub struct Epd {
    temp: i32
}

impl Epd {
    pub fn new(reader: &mut Read) -> Result<Epd, String> {
        unimplemented!();
    }
}
