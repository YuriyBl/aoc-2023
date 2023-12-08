use std::fs::File;
use std::io::{self, BufRead};

// Define a trait for the callback function
pub trait LineCallback {
    fn handle_line(&mut self, line: &str);
}

// Implement the trait for a closure
impl<F: FnMut(&str)> LineCallback for F {
    fn handle_line(&mut self, line: &str) {
        (self)(line);
    }
}

// Function to read a file line by line and invoke the callback
pub fn process_file<T: LineCallback>(file_path: &str, mut callback: T) {
    let file = File::open(file_path).unwrap();
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        callback.handle_line(&line);
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
