fn main() {
    for i in (1..101).rev() {

        match (i%3, i%5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("buzz"),
            (_,_) => println!("{}", i)
        }
    }
}
//
//        if el % 3 == 0 && el % 5 == 0 {
//            println!("Fizz Buzz = {}", el);
//        } else if el % 5 == 0 {
//            println!("buzz = {}", el);
//        } else if el % 3 == 0 {
//            println!("fizz = {}", el);
//        }
//    }
