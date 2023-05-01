#![allow(dead_code)]
#![allow(unused_imports)]
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

fn sanitize() {
    let flagged_words = get_flagged_words("./input/flagged_words.txt");
    let flagged_root = vec!["test"];
    let flagged_prefix = vec!["test"];
    let flagged_suffix = vec!["test"];

    let sanitized_file = "./output.txt";
    let mut output = File::create(sanitized_file).unwrap();

    if let Ok(lines) = read_lines("./input/words.txt") {
        'outer: for line in lines {
            if let Ok(word) = line {
                let word = word.to_lowercase();
                if !flagged_words.contains(&word) {
                    for r in &flagged_root {
                        if word.contains(r) {
                            continue 'outer;
                        }
                    }
                    for r in &flagged_suffix {
                        if word.ends_with(r) {
                            continue 'outer;
                        }
                    }
                    for r in &flagged_prefix {
                        if word.ends_with(r) {
                            continue 'outer;
                        }
                    }
                    output.write_all(word.as_bytes()).unwrap();
                    output.write_all(b"\n").unwrap();
                }
            }
        }
    }
    output.flush().unwrap();
}

fn get_flagged_words(path: &str) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(w) = line {
                res.push(w.to_lowercase());
            }
        }
    }
    res
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        sanitize();
        print!("done");
    }
}
