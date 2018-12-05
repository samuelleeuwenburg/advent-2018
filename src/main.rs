use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;

fn main() {
    day1();
}


fn day1() {
    let data = open_file(Path::new("./input/day1.txt"));
    let mut iter = data.split_whitespace();

    let mut frequency = 0i32;
    loop {
        match iter.next() {
            Some(word) => {
                match word.parse::<i32>() {
                    Err(why) => panic!("{} : {}", word, why),
                    Ok(i) => frequency += i
                }
            }
            None => break
        }
    }
    
    println!("{}", frequency)
}

fn open_file(path : &Path) -> String {
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open file {}: {}", path.display(), why.description()),
        Ok(file) => file,
    };

    let mut string = String::new();

    match file.read_to_string(&mut string) {
        Err(why) => panic!("can't read file {}", why),
        Ok(_) => string,
    }
}
