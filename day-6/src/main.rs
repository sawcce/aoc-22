use std::fs::read_to_string;
use std::env::args;

fn main() -> Result<(), &'static str> {
    let path = args().nth(1).unwrap_or("input.txt".to_string());
    let input = read_to_string(path).unwrap();

    // Part 1
    let marker = find_start(&input, 4)?;
    println!("[Part 1] Marker found at {}!", marker.clone());

    // Part 2
    let marker = find_start(&input, 14)?;
    println!("[Part 2] Marker found at {}!", marker.clone());

    Ok(())
}

fn find_start(input: &String, amount: usize) -> Result<usize, &'static str> {
    let mut stack = Vec::new();

    for (index, current) in input.chars().enumerate() {
        if stack.contains(&current) {
            let conflict_position = stack.iter().position(|value| value == &current);
            let new_start = conflict_position.unwrap() + 1;
            stack = stack[new_start..].to_vec();
            stack.push(current);
        } else if stack.len() == amount - 1 {
            return Ok(index + 1);
        } else {
            stack.push(current);
        }
    }

    Err("Could not find start pattern in input!")
}
