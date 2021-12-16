pub fn get_puzzle_input() -> core::str::Lines<'static> {
    include_str!("../inputs/day_three_a.txt")
        .lines()
}

struct Diagnostic {
    // a zero-indexed width of the data lines
    width: usize,
    // our calculated gamma rate, if performed yet
    gamma: Option<usize>,
    // our calculated epsilon rate, if performed yet
    epsilon: Option<usize>,
    // our calculated oxygen generator rating, if avail
    oxygen: Option<usize>,
    // our calculated co2 scrubber rating, if avail
    scrubber: Option<usize>,
    // our calculated life support rating, if avail
    life_support: Option<usize>,
    // our final answer for a, if prepared yet
    answer_a: Option<usize>,
    // our final answer for b, if prepared yet
    answer_b: Option<usize>,
    //
    zeros: Option<Vec<usize>>,
    ones: Option<Vec<usize>>,
}

impl Diagnostic {

    fn new() -> Diagnostic {
        Diagnostic {
            width: 12,
            gamma: None,
            epsilon: None,
            oxygen: None,
            scrubber: None,
            life_support: None,
            answer_a: None,
            answer_b: None,
            zeros: None,
            ones: None,
        }
    }

    fn get_gamma_and_epsilon(self: &mut Self) -> &mut Self {

        let puzzle_input = get_puzzle_input();

        let mut zeros = vec![0; self.width];
        let mut ones = vec![0; self.width];

        for line in puzzle_input {

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

        // add the zeros and ones to our struct for later use
        self.zeros = Some(zeros);
        self.ones = Some(ones);

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

    fn get_oxygen_rating(self: &mut Self) -> &mut Self {

        // create a vector of the highest bit in each column
        // 1's win any ties
        let o = self.ones.as_ref().expect("No ones are found.");
        let z = self.zeros.as_ref().expect("No zeroes are found.");
        let mut highest_values: Vec<usize> = Vec::new();
        for (i, count_of_ones) in o.iter().enumerate() {
            if count_of_ones.ge(&z[i]) {
                highest_values.push(1);
            } else {
                highest_values.push(0);
            }
        };

        let mut input: Vec<&str> = get_puzzle_input().into();
        let mut current_index = highest_values.iter();
        
        while input.len().ne(&1) {
            let c = current_index.next().expect("Index could not be set.");
            input.filter(|&x| (x[(c -1)..*c].parse::<usize>().unwrap()).eq(highest_values.iter().nth(*c).expect("No index exists."))).collect();
        }

        println!("filtered input: {:#?}", &input);

        todo!()
    }

    fn get_scrubber_rating(self: &mut Self) -> &mut Self {todo!() }

    fn get_life_support_rating(self: &mut Self) -> &mut Self {todo!()}

}

pub fn run() {
    
    let mut diagnostic = Diagnostic::new();
    diagnostic.get_gamma_and_epsilon();
    println!(
        "Day 3A: Final Answer - {:#?}", 
        diagnostic.answer_a.expect("Unable to calculate answer - 3A."));

    diagnostic.get_oxygen_rating();
    diagnostic.get_scrubber_rating();
    diagnostic.get_life_support_rating();
    println!(
        "Day 3B: Final Answer - {:#?}",
        diagnostic.answer_b.expect("Unable to calculate answer - 3B."));
}