use std::env;
use std::fs::read_to_string;

static ASCII: &[char] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn main() {
    let mut args = env::args();

    let path = match args.nth(1) {
        Some(path) => path,
        None => panic!("Expected input path!"),
    };

    let input = read_to_string(path).unwrap();

    //let count = HashMap::new();

    /* Part 1 */
    let sum = input
        .lines()
        .map(|line| {
            let compartements = line.split_at(line.len() / 2);
            let common_index = compartements
                .0
                .find(|c| compartements.1.contains(c))
                .unwrap();
            let common_type = compartements.0.chars().nth(common_index).unwrap();

            return ASCII.iter().position(|c| c == &common_type).unwrap() as u32 + 1u32;
        })
        .reduce(|accum, val| accum + val);

    println!("{sum:?}");

    /* Part 2 */
    let mut iter = input.lines().into_iter();
    let mut sum = 0;

    while let Some(first) = iter.next() {
        let second = iter.next().unwrap();
        let third = iter.next().unwrap();

        let common_index = first
            .find(|c| second.contains(c) && third.contains(c))
            .unwrap();
        let common_type = first.chars().nth(common_index).unwrap();
        sum += ASCII.iter().position(|c| c == &common_type).unwrap() as u32 + 1u32;
    }

    println!("{sum:?}");
}
