
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use std::env;



fn main() {

    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let colid: usize = args[1].parse().expect("Failed to parse string to integer");
    let thefile = &args[2];

    if let Ok(lines) = read_lines(thefile) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            let fields: Vec<&str> = line.split('\t').collect();
            println!("{}", fields[colid]);
        }
    }
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}




