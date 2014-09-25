use std::io::{File, BufferedReader};
use std::str;

pub struct DataReader {
  reader: BufferedReader<File>
}

impl Iterator<String> for DataReader {
  fn next(&mut self) -> Option<String> {
    self.reader
      .read_until(',' as u8)
      .ok()
      .and_then(trim_markup)
  }
}

fn trim_markup(bytes: Vec<u8>) -> Option<String> {
  str::from_utf8(bytes.as_slice()).map(|slice| {
    slice.trim_chars(['"', ','].as_slice()).into_string()
  })
}

pub fn for_path(path_str: &str) -> DataReader {
  let path   = &Path::new(path_str);
  let file   = File::open(path).unwrap();
  let reader = BufferedReader::new(file);

  DataReader { reader: reader }
}
