use std::io::{BufReader, BufRead, Cursor, Read};

pub enum EpdOperand {
    Str(String),
    Move(String),
    Unsigned(u32),
    Integer(i32),
    Float(f32)
}

pub struct Epd {
    fen_string: String,
    operations: Vec<EpdOperand>
}

pub struct EpdReader<'a> {
    reader: &'a mut Read,
}

impl<'a> EpdReader<'a> {
    pub fn new(reader: &mut Read) -> EpdReader {
        EpdReader{reader: reader}
    }

    pub fn decode(&mut self) -> Result<Vec<Epd>, String> {
        let buf_read = BufReader::new(&mut self.reader);
        let mut ret: Vec<Epd> = Vec::new();

        for line in buf_read.lines() {
            let line = line.unwrap();
            let res = Epd::new(line);
            match res {
                Ok(e) => ret.push(e),
                Err(s) => return Err(s),
            }
        }

        Ok(ret)
    }
}

impl Epd {
    pub fn new(line: String) -> Result<Self, String> {
        return Err("Not implemented".to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufReader, BufRead, Cursor, Read};

    #[test]
    fn test_perft_line() {
        let mut read = Cursor::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - D1 20; D2 400; D3 8902; D4 197281; D5 4865609; D6 119060324;");

        let epd = EpdReader::new(&mut read).decode();

        assert!(epd.is_ok());
        let epd = epd.unwrap();
        assert!(epd.len() == 1);
    }
}
