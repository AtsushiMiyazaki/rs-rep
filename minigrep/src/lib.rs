use std::error::Error;
use std::fs;

use std::process;
mod utils;

pub fn run(args: &Vec<String>) -> Result<(), Box<dyn Error>> {
    if args.len() == 0 {
        utils::print_usage();
    }

    let config = Config::new(args).unwrap_or_else(|e| {
        println!("Args err: {}", e);
        utils::print_usage();
        process::exit(1);
    });

    let res = minigrep(&config)?;
    print_lines(&res);
    Ok(())
}

pub fn minigrep(conf: &Config) -> Result<Vec<String>, Box<dyn Error>> {
    let content = fs::read_to_string(&conf.filename)?;

    let res = match conf.flags {
        Flags {
            case_sensitive: true,
        } => search(&conf.target, &content),
        _ => search_case_insensitive(&conf.target, &content),
    };

    Ok(res)
}

fn search(target: &String, content: &String) -> Vec<String> {
    content.lines().filter(|l| l.contains(target)).map(|x| x.to_owned()).collect()
}

fn search_case_insensitive(target: &String, content: &String) -> Vec<String> {
    content.lines().filter(|l| l.to_lowercase().contains(&target.to_lowercase())).map(|x| x.to_owned()).collect()
}

fn print_lines(lines: &Vec<String>) {
    lines.iter().for_each(|x| println!("{}", x));
}

struct Flags {
    case_sensitive: bool,
}

impl Flags {
    pub fn new(flags: &Vec<String>) -> Result<Flags, &str> {
        let mut flg = Flags {
            case_sensitive: true,
        };

        for flag in flags.iter() {
            match flag.as_str() {
                "-i" => {
                    flg.case_sensitive = false;
                }
                _ => {
                    return Err("unknown option");
                }
            }
        }
        Ok(flg)
    }
}

pub struct Config {
    target: String,
    filename: String,
    flags: Flags,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Self, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let target = args[1].clone();
        let filename = args[2].clone();
        let flags = Flags::new(&args[3..].to_vec()).unwrap();

        Ok(Config {
            target,
            filename,
            flags,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_one_line() {
        let target = String::from("ally");
        let contents = String::from(
            "
this is 
it.
finally what
I 
expected.",
        );

        assert_eq!(vec!["finally what"], search(&target, &contents));
    }

    #[test]
    fn case_sensitive() {
        let query = String::from("duct");
        let contents = String::from(
            "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.",
        );

        assert_eq!(vec!["safe, fast, productive."], search(&query, &contents));
    }

    #[test]
    fn case_insensitive() {
        let query = String::from("rUsT");
        let contents = String::from(
            "\
Rust:
safe, fast, productive.
Pick three.
Trust me.",
        );

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(&query, &contents)
        );
    }

    #[test]
    fn minigrep_case_sensitive() {
        let mut conf = Config {
            target: String::from("D"),
            filename: String::from("test.txt"),
            flags: Flags {
                case_sensitive: true,
            },
        };

        assert_eq!(vec!["Duck Duck go."], minigrep(&conf).unwrap());

        conf.flags.case_sensitive = false;
        assert_eq!(
            vec!["safe, fast, productive.", "Duck Duck go."],
            minigrep(&conf).unwrap()
        );
    }
}
