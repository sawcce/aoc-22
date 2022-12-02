use crate::*;
use day_2::*;

pub(crate) fn process(parts: Vec<&str>) -> usize {
    return round(Pick::from(parts[0]), Outcome::from(parts[1]));
}

fn round(pick: Pick, outcome: Outcome) -> usize {
    let shape = if outcome == Outcome::Draw {
        pick
    } else if outcome == Outcome::Loss {
        BEATS[Into::<usize>::into(pick.clone()) - 1].clone()     
    } else {
        BEATEN[Into::<usize>::into(pick.clone()) - 1].clone()
    };
    
    let shape_score: usize = shape.into();
    
    return shape_score + outcome as usize;
}

