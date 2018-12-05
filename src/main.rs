use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    day1();
}

fn day1() {
    let data = open_file(Path::new("./input/day1.txt"));
    let mut iter = data.split_whitespace();
    let mut frequency = 0i32;
    let mut frequencies = HashSet::new();
    let mut first_repeat : Option<i32> = None;
    let mut loops = 0i32;

    loop {
        match iter.next() {
            Some(number) => {
                match number.parse::<i32>() {
                    Ok(i) => {
                        frequency += i;

                        if frequencies.contains(&frequency) {
                            first_repeat = Some(frequency);
                            break;
                        }

                        frequencies.insert(frequency);
                    }
                    Err(why) => panic!("{} : {}", number, why)
                }
            }
            None => {
                if loops > 1000 {
                    break;
                }

                loops += 1;

                iter = data.split_whitespace();
            }
        }
    }

    match first_repeat {
        Some(i) => println!("first repeat was {}, found after repeating the input {} times", i, loops),
        None => println!("no repeat found"),
    }
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
