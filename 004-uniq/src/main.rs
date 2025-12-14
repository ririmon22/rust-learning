use std::env;

fn uniqe<'a>(contents:&'a str) -> Vec<&'a str> {
    let mut result:Vec<&'a str> = Vec::new();
    let mut prev: Option<&'a str> = None;
    for line in contents.lines() {
        if Some(line) != prev{
            result.push(line);
        }
        prev = Some(line);
    }
    result
}
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let filename = env::args().nth(1).ok_or("no filename")?;
    let contents = std::fs::read_to_string(filename)?;
    
    let lines = uniqe(&contents);
    for line in lines{
        println!("{}", line);
    }
    Ok(())
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn uniq_basic() {
        let contents = "\
a
a
b
b
b
c
c";
        let expected = vec!["a","b","c"];
        assert_eq!(uniqe(contents), expected);
    }
}