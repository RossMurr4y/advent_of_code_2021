const WIDTH: usize = 12;

/// Determining the gamma and epsilon values is very simmilar logic.
/// Defining an Enum with variants of each allows us to write a function
/// that will accept either, and perform only the relevant actions for
/// the provided variant.
#[derive(Debug)]
enum DiagnosticRate {
    Gamma(usize),
    Epsilon(usize),
}

/// A struct to represent our Diagnostic.
/// This uses the DiagnosticRate enum variants for types.
#[derive(Debug)]
struct Diagnostic {
    data: String,
    gamma_rate: DiagnosticRate,
    epsilon_rate: DiagnosticRate,
    power_consumption: usize,
}

/// Implements the associated functions & methods of the Diagnostic struct
impl Diagnostic {

    /// allocate the input binary data for the diagnostic
    fn get_data(&mut self) -> &mut Self {
        self.data = std::fs::read_to_string("src/inputs/day_three_a.txt")
            .expect("Unable to retrieve input data.");
        self
    }

    /// Generate a Diagnostic's rate based on which enum variant is provided
    fn generate_rate(&mut self, rate: DiagnosticRate) -> &mut Self {
        let lines = self.data.lines();

        // instantiate result vectors for later comparison
        let mut zeroes: Vec<usize> = vec![0; WIDTH];
        let mut ones: Vec<usize> = vec![0; WIDTH];

        // loop over our lines and increment our result vectors as we go
        for line in lines {
            let mut char_indicies = line.char_indices();
            for (idx, char_value) in char_indicies {
                match char_value {
                    '0' => zeroes[idx] += 1,
                    '1' => ones[idx] += 1,
                    // there should never be anything found other than a zero or one.
                    _ => unreachable!(),
                }
            }
        }

        // compare our two result vectors to create our gamma value
        let mut gamma: String = String::new();
        for idx in 0..WIDTH {
            match rate {
                DiagnosticRate::Gamma(_) => {
                    if zeroes[idx] > ones[idx] { gamma.push_str("0")} else { gamma.push_str("1") };
                },
                DiagnosticRate::Epsilon(_) => {
                    if zeroes[idx] > ones[idx] { gamma.push_str("1")} else { gamma.push_str("0") };
                },
            }
        }

        match rate {
            DiagnosticRate::Gamma(_) => {
                // set our gamma value by parsing our string slice into radix 2
                self.gamma_rate = DiagnosticRate::Gamma(
                    usize::from_str_radix(&gamma, 2)
                        .expect("Unable to parse gamma value into radix 2.")
                )
            },
            DiagnosticRate::Epsilon(_) => {
                // set our gamma value by parsing our string slice into radix 2
                self.epsilon_rate = DiagnosticRate::Epsilon(
                    usize::from_str_radix(&gamma, 2)
                        .expect("Unable to parse epsilon value into radix 2.")
                )
            },
        }

        self
    }

    fn get_power_consumption(&mut self) -> &mut Self {
        if let DiagnosticRate::Gamma(gamma) = self.gamma_rate {
            if let DiagnosticRate::Epsilon(epsilon) = self.epsilon_rate {
                self.power_consumption = gamma * epsilon;
            };
        }
        self
    }


    fn new() -> Diagnostic {
        let mut diagnostic = Diagnostic {
            data: Default::default(),
            gamma_rate: DiagnosticRate::Gamma(Default::default()),
            epsilon_rate: DiagnosticRate::Epsilon(Default::default()),
            power_consumption: Default::default(),
        };

        diagnostic.get_data()
            .generate_rate(DiagnosticRate::Gamma(Default::default()))
            .generate_rate(DiagnosticRate::Epsilon(Default::default()))
            .get_power_consumption();

        diagnostic
    }

}

/// Our only exported/public function, this will run the Day 3 puzzle solutions
pub fn run() {
    // instantiate our Diagnostic
    let diagnostic = Diagnostic::new();

    println!("Day 3A: Final Answer - {}", diagnostic.power_consumption);
}