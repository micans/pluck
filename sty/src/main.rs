
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::any::type_name_of_val;

use std::env;



fn main() {

    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let colid: usize = args[1].parse().expect("Failed to parse string to integer");
    let thefile = &args[2];

    if let Ok(mut lines) = read_lines(thefile) {

        let myline = lines.next();
        println!("myline {}", type_name_of_val(&myline));

        let val = myline.expect("Header line");
        println!("val {}", type_name_of_val(&val));

        let val2 = val.expect("oops not Ok 2");
        println!("val2 {}", type_name_of_val(&val2));



        let v2 = lines.next().expect("second line").expect("second value");
        println!("v2 {}", type_name_of_val(&v2));
        println!("result {}", v2);

        // let val3 = val.expect("oops not Ok 3");
        // println!("val3 {}", type_name_of_val(&val3));

        // if let Some(h) = myline {
        //   match h {
        //     Ok(v) => {
        //       println!("result is {}", v);
        //       println!("{}", type_name_of_val(&v));
        //     }
        //     Err(e) => {
        //       println!("error: {}", e);
        //       std::process::exit(1)
        //     }
        //   }
        // }

        // Consumes the iterator, returns an (Optional) String
        for line in lines.map_while(Result::ok) {
            let fields: Vec<&str> = line.split('\t').collect();
            println!("Column value is {}", fields[colid]);
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




