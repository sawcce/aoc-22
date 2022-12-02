mod part1;
mod part2;

use std::env;
use std::fs::read_to_string;

fn main() {
    let mut args = env::args();
    
    println!("{args:?}");
    
    let (input, process_function) = match args.len() {
        2 => {
            let path = args.nth(1).unwrap();
            let text = read_to_string(path).unwrap();
            let method: fn(Vec<_>) -> _ = part1::process;
            
            (text, method)
        },
        3 => {
            let path = args.nth(1).unwrap();
            let text = read_to_string(path).unwrap();
            let part = args.nth(0).unwrap();
            
            let method = match part.as_str() {
                "1" => part1::process,
                "2" => part2::process,
                _ => panic!("Expected either 1 or 2 for the second argument!")
            };
            
            (text, method)
        },
        _ => panic!("Expected 1 or 2 arguments!"),
    };
    
    
    
    let result = input
        .lines()
        .map(|line| {
            let parts = line.split(" ").collect::<Vec<&str>>();
            return process_function(parts);
        })
        .reduce(|accum, item| accum + item);
    
    println!("{result:?}");
}