pub fn get_puzzle_input() -> core::iter::Enumerate<core::str::Lines<'static>> {
    include_str!("../inputs/day_three_a.txt")
        .lines()
        .enumerate()
}

struct Diagnostic {
    // a zero-indexed width of the data lines
    width: usize,
    // our calculated gamma rate, if performed yet
    gamma: Option<usize>,
    // our calculated epsilon rate, if performed yet
    epsilon: Option<usize>,
    // our final answer for a, if prepared yet
    answer_a: Option<usize>,
    // our final answer for b, if prepared yet
    answer_b: Option<usize>,
}

impl Diagnostic {

    fn new() -> Diagnostic {
        Diagnostic {
            width: 12,
            gamma: None,
            epsilon: None,
            answer_a: None,
            answer_b: None,
        }
    }

    fn add_gamma_and_epsilon(self: &mut Self) -> &mut Self {

        let puzzle_input = get_puzzle_input();

        let mut zeros = vec![0; self.width];
        let mut ones = vec![0; self.width];

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

        // calculate and set gamma value
        self.gamma = Some(usize::from_str_radix(&gamma_rate_binary, 2)
            .expect("Error calculating the gamma_rate_decimal"));

        // calculate and set epsilon value
        self.epsilon = Some(usize::from_str_radix(&epsilon_rate_binary, 2)
            .expect("Error calculating the epsilon_rate_decimal"));

        // calculate and set answer_a value
        // these are safe to unwrap as we added the Some() ourselves
        self.answer_a = Some(self.gamma.unwrap() * self.epsilon.unwrap());

        // return our updated struct
        self
    }

}



pub fn run() {
    
    let mut diagnostic = Diagnostic::new();
    diagnostic.add_gamma_and_epsilon();
    println!("Day 3A: Final Answer - {:#?}", diagnostic.answer_a.expect("Unable to calculate answer."));
}