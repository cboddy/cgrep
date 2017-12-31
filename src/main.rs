extern crate glob;
extern crate regex;
use std::env;
use std::fs;
use std::path::Path;
use std::io::prelude::*;
use regex::Regex;

#[derive(Debug)]
struct Config {
    search: Regex,
    path: String
}


//parse config from sys argc
fn parse_config() -> Config {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Usage: cgrep search glob_path");
    };

    Config {
        search: Regex::new(&args[1]).unwrap(),
        path: args[2].clone()
    }
}

// read contents of file @ path
fn read(path: &Path) -> String {
    let mut f =  fs::File::open(path).unwrap();
    let mut s = String::new();                          
    f.read_to_string(&mut s).unwrap();
    s
}

//search for regex-search line-by-line 
fn search_lines(search: &Regex, from_body: &String) -> Vec<String> {
    let x: Vec<&str> = from_body.split("\n").collect();
    x.iter()
        .filter(|&line| search.is_match(line))
        .map(|&line| line.to_string())
        .collect()
}



fn main() {
    // parse config
    let config = parse_config();
    println!("config {:?}",  config);
    // glob input path
    for path in glob::glob(&config.path).unwrap().filter_map(Result::ok) {
        let file_contents = read(&path);
        // search matched file for search
        let match_lines = search_lines(&config.search, &file_contents);
        // print matched lines 
        for line in match_lines{
            println!("{}:{}", &path.display(), line);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{Regex, search_lines};
    #[test]
    fn test_search_lines() {
        let matches = search_lines(&Regex::new("z").unwrap(),
                                          &String::from("xxxx\nyyy\n zz"));
        assert_eq!(vec![" zz".to_string()],
                   matches);
    }

    #[test]
    fn test_search_lines_re() {
        let matches = search_lines(&Regex::new(r"\w{2} $").unwrap(),
                                          &String::from("xxxx\nyyy\n zz "));
        assert_eq!(vec![" zz ".to_string()],
                   matches);
    }
}
