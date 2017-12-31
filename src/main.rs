use std::env;
use std::fs;
use std::io::prelude::*;

#[derive(Debug)]
struct Config {
    search_term: String,
    path: String
}

// read contents of file @ path
fn read(path: &str) -> String {
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


fn parse_config() -> Config {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("Usage: cgrep pattern path");
    };

    Config {
        search_term: args[1].clone(),
        path: args[2].clone()
    }
}

fn main() {
    let config = parse_config();
    println!("config {:?}",  config);
    let file_contents = read(&config.path);
    let match_lines = search_lines(&config.search_term, &file_contents);
    println!("{:?}", &match_lines);
}


mod tests {
    #[test]
    fn test_search() {
        let matches = super::search_lines(&String::from("z"),
                                   &String::from("xxxx\nyyy\n zz"));
        assert_eq!(vec![" zz".to_string()],
                   matches);
    }

}
