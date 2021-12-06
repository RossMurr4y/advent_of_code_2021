pub fn get_puzzle_input() -> core::iter::Enumerate<core::str::Lines<'static>> {
    include_str!("../inputs/day_three_a.txt")
        .lines()
        .enumerate()
}

pub fn run() {

    const WIDTH: usize = 12;
    
    let puzzle_input = get_puzzle_input();

    let mut zeros = vec![0; WIDTH];
    let mut ones = vec![0; WIDTH];

    for (line_idx, line) in puzzle_input {

        let line_bytes = line.as_bytes().iter().enumerate();

        for (i, byte) in line_bytes {
            
            // b'0' is matching on a byte-literal.
            match byte {
                b'0' => {
                    zeros[i] = zeros[i] + 1;
                },
                b'1' => {
                    ones[i] = ones[i] + 1;
                },
                b'\n' => { 
                    // do nothing on empty lines
                },
                _ => {
                    // panic if anything else is found.
                    panic!("invalid number found");
                },
            }
        }
    }

    // calculate the gamma
    let mut gamma_rate_binary = String::new();
    let mut epsilon_rate_binary = String::new();
    for (i, &count) in zeros.iter().enumerate() {
        if count.gt(&ones[i]) {
            gamma_rate_binary.push('0');
            epsilon_rate_binary.push('1');
        } else {
            gamma_rate_binary.push('1');
            epsilon_rate_binary.push('0');
        }
    }

    let gamma_rate_decimal = usize::from_str_radix(&gamma_rate_binary, 2)
        .expect("Error calculating the gamma_rate_decimal");
    let epsilon_rate_decimal = usize::from_str_radix(&epsilon_rate_binary, 2)
        .expect("Error calculating the epsilon_rate_decimal");

    // loop over our zeros and ones and compare each index.
    // The one with the highest value becomes the index of
    // our result - ie if zeroes is higher, that index has
    // a value of 0.
    println!("Day 3A: Zeros: {:#?}", zeros);
    println!("Day 3A: Ones: {:#?}", ones);
    println!("Day 3A: Final Answer - {:#?}", gamma_rate_decimal * epsilon_rate_decimal);
}