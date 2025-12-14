use std::env;
use std::fs::File;
use std::io::prelude::*;

fn count_lines(s: &str) -> usize {
    s.lines().count()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];
    println!("Searching for {}", query);
    println!("In file {}", filename);
    
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something went wrong reading the file");
    
    println!("With text:\n{}", contents);
    
    let count = count_lines(&contents);
    println!("行数は{}",count);
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn count_lines_basic() {
        let s = "a\nb\nc";
        assert_eq!(count_lines(s), 3);
    }
}