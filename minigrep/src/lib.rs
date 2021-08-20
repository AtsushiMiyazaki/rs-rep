use std::error::Error;
use std::fs;

pub fn minigrep(conf: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(conf.filename)?;
    search(conf.target, content);
    Ok(())
}

fn search(target: String, content: String) -> Vec<String> {
    let mut res = vec!();
    let lines = content.lines();
    for line in lines {
        if line.contains(&target) {
            res.push(line.to_owned());
        }
    }

    res
}

pub struct Config {
    target: String,
    filename: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let target = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { target, filename })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_one_line() {
        let target = "ally";
        let contents = "
this is 
it.
finally what
I 
expected.";

        assert_eq!(vec!["finally what"], search(target.to_owned(), contents.to_owned()));
    }
}
