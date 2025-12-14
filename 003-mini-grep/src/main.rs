use std::env;

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matching_lines:Vec<&'a str> = Vec::new();
    let lines = contents.lines();
    for line in lines{
        if line.contains(query){
            matching_lines.push(line);
        }
    }
    matching_lines
}
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let query = env::args().nth(1).ok_or("no query given")?;
    let filename = env::args().nth(2).ok_or("no filename given")?;
    let contents = std::fs::read_to_string(filename)?;
    
    let lines = search(&query, &contents);
    
    for res in lines{
        println!("{}", res);
    }
    Ok(())
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn search_test() {
        let query = "Rust";
        let contents = "\
Rust is fast.
Rust is memory-safe.
Trust me, Rust is great.
This language is amazing.
We trust Rust.";
        let expected = vec![
            "Rust is fast.",
            "Rust is memory-safe.",
            "Trust me, Rust is great.",
            "We trust Rust.",
        ];
        let lines = search(query, contents);
        assert_eq!(lines, expected);
    }
}