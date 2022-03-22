use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn print_type_of<T>(_: &T) {
   println!("{}", std::any::type_name::<T>())
}

fn main() {

    let mut old = 0;
    let mut increases = 0;
    let mut decreases = 0;

    if let Ok(lines) = read_lines("./test_input.txt") {

        print_type_of(&lines);

        for line in lines {
            if let Ok(measurement) = line {

                //Coerce String to unsigned 32 bit int
                let int_measurement: u32 = measurement.parse().unwrap();




                if int_measurement > old {
                    increases = increases + 1;
                } else {
                    decreases = decreases + 1;
                }

                old = int_measurement;

            }
        }
    }


    println!("increases = {} and decreases = {}", increases, decreases);

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


//fn analyze() {
//
//    let measurements = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
//
//    let mut idx: usize = 1;
//    let mut increases = 0;
//    let mut decreases = 0;
//
//
//    while idx < measurements.len() {
//
//        let curr_measurement = measurements[idx];
//        let prev_measurement = measurements[idx - 1];
//
//        if curr_measurement > prev_measurement {
//            increases = increases + 1;
//
//        } else {
//            decreases = decreases + 1;
//        }
//
//        idx = idx + 1;
//    }
//
//    println!("increases = {} and decreases = {}", increases, decreases);
//}

//Integer overflow issue because I tried to subtract 1 from 0 which meant the integer was wrapping
//back to the previous high number for whatever the type was i declaring, eg i32
//
//Tried to make http request to puzzle input, unable to because its an auth experience!
