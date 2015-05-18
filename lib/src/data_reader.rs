use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use std::borrow::ToOwned;
use std::str;

pub struct DataReader {
    reader: BufReader<File>
}

impl Iterator for DataReader {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        let mut buffer = Vec::new();

        return self.reader.read_until(',' as u8, &mut buffer).ok().and_then(|bytes| {
            if bytes > 0 {
                trim_markup(buffer)
            } else {
                None
            }
        });
    }
}

fn trim_markup(bytes: Vec<u8>) -> Option<String> {
    let chars: &[char] = &['"', ','];
    str::from_utf8(&*bytes).ok().map(|slice| {
        slice.trim_matches(chars).to_owned()
    })
}

pub fn for_path(path_str: &str) -> DataReader {
    let path   = Path::new(path_str);
    let file   = File::open(path).unwrap();
    let reader = BufReader::new(file);

    DataReader { reader: reader }
}
