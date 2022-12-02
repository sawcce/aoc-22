use crate::*;
use day_2::*;

pub(crate) fn process(parts: Vec<&str>) -> usize {
    return round(Pick::from(parts[0]), Pick::from(parts[1]));
}

fn round(opponent: Pick, other: Pick) -> usize {
    let shape_score: usize = other.clone().into();
    
    return shape_score + if BEATS[Into::<usize>::into(opponent.clone()) - 1] == other  {
        0
    } else if opponent == other {
        3
    } else {
        6
    }
}
