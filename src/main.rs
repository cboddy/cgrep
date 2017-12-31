extern crate glob;
use std::env;
use std::fs;
use std::path::Path;
use std::io::prelude::*;

#[derive(Debug)]
struct Config {
    pattern: String,
    path: String
}


//parse config from sys argc
fn parse_config() -> Config {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Usage: cgrep pattern glob_path");
    };

    Config {
        pattern: args[1].clone(),
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

//search for regex-pattern line-by-line 
fn search_lines(pattern: &String, from_body: &String) -> Vec<String> {
    let mut x: Vec<&str> = from_body.split("\n").collect();
    x.retain(|&line| line.matches(pattern).count() !=  0);
    x.iter().map(|&line| line.to_string()).collect()
}



fn main() {
    // parse config
    let config = parse_config();
    println!("config {:?}",  config);
    // glob input path
    for path in glob::glob(&config.path).unwrap().filter_map(Result::ok) {
        let file_contents = read(&path);
        // search matched file for pattern
        let match_lines = search_lines(&config.pattern, &file_contents);
        // print matched lines 
        for line in match_lines{
            println!("{}:{}", &path.display(), line);
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_search_lines() {
        let matches = super::search_lines(&String::from("z"),
                                          &String::from("xxxx\nyyy\n zz"));
        assert_eq!(vec![" zz".to_string()],
                   matches);
    }
}
