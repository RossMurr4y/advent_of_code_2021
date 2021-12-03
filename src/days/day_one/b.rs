use crate::day_one::a::get_input_data;
use std::iter::Sum;

/// A function to calculate the window increments
fn sliding_window_increments(measurements: Vec<i32>) -> i32 {

    // init the count
    let mut count = 0;
    
    // loop over a "window" of our measurements
    // "windows" are slices of a fixed size - in this case 4:
    // the total of the first 3 are our first measurement
    // the total of the last 3 are our second measurement
    for window in measurements.windows(4) {
        let first_window: i32 = Sum::sum(vec!(window[0], window[1], window[2]).iter());
        let second_window: i32 = Sum::sum(vec!(window[1], window[2], window[3]).iter());

        if first_window < second_window { 
            count += 1;
            println!("Increment found: First - {}, Second - {}, NewTotal: {}", first_window, second_window, count);
        } else {
            println!("No increment found.");
        }

    }

    println!("Total Window Increments: {}", count);
    count
}

pub fn run() {
    // retrieve data from day 1
    let data = get_input_data();

    // eval
    sliding_window_increments(data);
}