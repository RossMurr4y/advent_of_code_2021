use std::fs;
use std::io::{self, BufRead, Read, Seek, IoSliceMut, BufReader, Bytes};
use std::ops::{DerefMut, Deref};
use std::path::Path;

struct Diagnostic {
    gamma_rate: Option<Vec<u8>>,
    epsilon_rate: Option<i32>,
    binary_data: Option<Vec<u8>>,
    path: String,
}

impl Diagnostic {

    /// Instantiate a Diagnostic.
    fn from(path: String) -> Self {
        Diagnostic {
            gamma_rate: None,
            epsilon_rate: None,
            binary_data: None,
            path: path,
        }
    }

    fn get_gamma_rate(&mut self, lines: Vec<&[u8]>) -> &Self {

        // we create two vectors.
        // As we loop over each line, each bit will be evaluated.
        // if its a 1, it will be added to the vector for ones.
        // if its a 0, it will be added to the vector for zeros.
        // Then we compare the matching indexes for both vectors
        // determine the gamma rate.
        let mut zeros = [0; 13];
        let mut ones = [0; 13];

        for line in lines {


            let utf = String::from_utf8(line.deref().to_vec()).expect("error");

            for (idx, char) in utf.chars().enumerate() {
                match char as u32 {
                    0 => {
                        zeros[idx] += 1;
                        //zeros[index] += 1 
                    },
                    1 => { 
                        ones[idx] += 1;
                        //ones[index] += 1
                    },
                    _ => { println!("An invalid binary value was found: {}", char) }
                }
            }
        }
        
        let mut gamma: Vec<u8> = Vec::new();
        for i in [0..12] {
            if zeros[i.clone()] > ones[i] {
                gamma.push(0)
            } else {
                gamma.push(1)
            }
        }

        self.gamma_rate = Some(gamma);
        self
    }

    fn get_power_consumption(diag: Diagnostic) -> i32 {
        //diag.gamma_rate.expect("No gamma rate") * diag.epsilon_rate.expect("No epsilon rate")
        todo!()
    }
}

/// Retrieves an individual line of data
/// This is helpful so we don't have to handle
/// splitting out all the lines individually
fn get_puzzle_line(path: &Path, line: u64) -> String {

    // init a "cursor" within the file.
    // We move this around to specify where we
    // will read data into the buffer from.
    // - There are 13 bytes per line.
    // - file lines are zero-indexed, so subtract one.
    let cursor: u64 = 13 * (line - 1);

    // open our file
    let mut f = fs::File::open(path)
        .expect("Cannot reaad day_three_a input.");

    // position our "cursor"
    f.seek(io::SeekFrom::Start(cursor));

    // init our reader and buffer
    let mut reader = io::BufReader::new(f);
    let mut buffer = String::new();

    // use our reader to pull data into the buffer
    reader.read_line(&mut buffer)
        .expect("Error reading line into the buffer.");

    // return the content of be buffer.
    buffer
}

/// Read the puzzle input data, push it into a string,
/// and split it up by lines into a vector of strings.
fn get_puzzle_data(path: &Path) -> Vec<&[u8]> {

    // open our file
    let mut f = fs::File::open(path)
        .expect("Cannot reaad day_three_a input.");

    // init our reader and buffer
    let mut reader = io::BufReader::new(f);
    let mut buffer = String::new();

    // use our reader to pull data into the buffer
    reader.read_to_string(&mut buffer)
        .expect("Error reading file into the buffer.");

    let mut data: Vec<&[u8]> = Vec::new();

    for line in buffer.lines() {
        data.push(line.as_bytes())
    }
    // return the vector data
    data.clone().to_owned()
}

/// Determines how many lines there are in our puzzle input.
/// Clearly its 1000 but this is more dynamic.
fn get_puzzle_line_length(path: &Path) -> usize {
    fs::read(path)
        .expect("Cannot read day_three_a input.")
        .len()
}

pub fn run() {
    
    let mut input = String::from("src/inputs/day_three_a.txt");
    let path = Path::new(&input);
    let lines = get_puzzle_data(path);

    // instantiate our diagnostic data
    let mut diag = Diagnostic::from(input.clone());
    diag.get_gamma_rate(lines);

    panic!("Testing: {:#?}", diag.gamma_rate);
    //d.get_puzzle_data();
    //d.get_gamma_rate();

    //println!("Gamma rate is: {:#?}", d.gamma_rate);

    // determine gamma

    // determine epsilon

    // evaluate power consumtion

}