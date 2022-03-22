use std::fs::File;
use std::io::{BufRead, BufReader};

//fn print_type_of<T>(_: &T) {
//   println!("{}", std::any::type_name::<T>())
//}

fn main() {

    let mut old = 0;
    let mut increases = 0;
    let mut decreases = 0;

    let input = "./real_input.txt";

    let file = File::open(input).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let measurement: u32 = line.parse().unwrap();


        if index > 0 && measurement > old {
            increases = increases + 1;
        } else if index > 0 && measurement < old {
             decreases = decreases + 1;
        }

        old = measurement;

    }

    println!("increases = {} and decreases = {}", increases, decreases);

}

//Integer overflow issue because I tried to subtract 1 from 0 which meant the integer was wrapping
//back to the previous high number for whatever the type was i declaring, eg i32
//
//Tried to make http request to puzzle input, unable to because its an auth experience!
