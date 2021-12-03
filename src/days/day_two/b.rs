
// import input data from part a
use crate::days::day_two::a::{Direction, get_input_data};

/// A helper structure to define a position
struct Position {
    x: i32,
    y: i32,
    aim: i32,
}

/// adds a default structure for a position
impl Default for Position {
    fn default() -> Self {
        Self { x: 0, y: 0, aim: 0 }
    }
}

/// We define a trait called `Movement` that is common amongst
/// all the possible directions.
/// 
/// Each direction will then be forced to implement a `advance`
/// function (move is a keyword apparently) with a matching signature. 
/// This will make it simple to call each of them. 
/// It will be up to each implementation to control what it does.
pub trait Movement {
    /// advance will define how movement of a position should occurr
    /// for any given direction
    fn advance(direction: Direction, position: Position) -> Position;
}

impl Movement for Direction {
    fn advance(direction: Self, position: Position) -> Position {

        // init the new_position from the old one
        let mut new_position = position;
        
        match direction {
            Direction::Forward(distance) => {
                new_position.x += distance;
                new_position.y += (new_position.aim * distance);
            },
            Direction::Down(distance) => { 
                new_position.aim += distance; 
            },
            Direction::Up(distance) => { 
                new_position.aim -= distance;
            },
        };

        // return our updated position
        new_position
    }
}

pub fn run() {

    // plug in our existing plotted course
    let planned_course = get_input_data();

    // our coordinates start at 0, 0
    let mut position = Position::default();

    // loop over the course, performing `advance` on each
    for dir in planned_course {
        position = Direction::advance(dir, position);
    }

    let answer = position.x * position.y;

    println!("Day 2B: Final Coordinates: {}, {}", position.x, position.y);

    println!("Day 2B: Final Answer: {}", answer);
}
