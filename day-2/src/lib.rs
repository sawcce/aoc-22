#[derive(Debug, Clone, PartialEq)]
pub enum Pick {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, PartialEq)]
#[repr(usize)]
pub enum Outcome {
    Win = 6,
    Loss = 0,
    Draw = 3,
}

pub const BEATS: &[Pick] = &[Pick::Scissors, Pick::Rock, Pick::Paper];
pub const BEATEN: &[Pick] = &[Pick::Paper, Pick::Scissors, Pick::Rock];

impl Into<usize> for Pick {
    fn into(self) -> usize {
        match self {
            Pick::Rock => 1,
            Pick::Paper => 2,
            Pick::Scissors => 3,
        }
    }
}

impl From<&str> for Outcome {
    fn from(value: &str) -> Outcome {
        match value {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Unexpected combination"),
        }
    }
}

impl From<&str> for Pick {
    fn from(value: &str) -> Pick {
        match value {
            "A" | "X" => Pick::Rock,
            "B" | "Y" => Pick::Paper,
            "C" | "Z" => Pick::Scissors,
            _ => panic!("Unexpected combination"),
        }
    }
}
