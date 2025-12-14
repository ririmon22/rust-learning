use std::env;
use std::fs::File;
use std::io::prelude::*;

fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = env::args().nth(1).ok_or("no filename given")?;
    
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something wrong reading the file");
    println!("{}", count_words(&contents));
    
    Ok(())
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn count_words_basic() {
        let s = "a\nb c";
        
        assert_eq!(count_words(s), 3);
    }
}