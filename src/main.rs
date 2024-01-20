use std::io::{self, BufRead};
use std::collections::HashMap;
use std::process;

fn main() {
  let stdin = io::stdin();
  let mut lines = stdin.lock().lines();

  let mut delimiter_counts: HashMap<char, usize> = HashMap::new();

  if let Some(Ok(first_line)) = lines.next() {
    let delimiters = vec![' ', '\t', ',', ';', ':', '|'];

    for delimiter in delimiters {
      let count = first_line.matches(delimiter).count();
      if count > 0 {
        delimiter_counts.insert(delimiter, count);
      }
    }
  }

  let mut most_common_delimiter: Option<(char, usize)> = None;
  for (delimiter, count) in delimiter_counts {
    if most_common_delimiter.is_none() || count > most_common_delimiter.unwrap().1 {
      most_common_delimiter = Some((delimiter, count));
    }
  }

  match most_common_delimiter {
    Some((delimiter, _)) => print!("{}", delimiter),
    None => {
        eprintln!("Could not determine the field separator");
        process::exit(1);
    },
  }
}
