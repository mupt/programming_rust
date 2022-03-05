fn main() {

    let mut n1 = 0;
    let mut n2 = 1;
    let mut next_term;

    for _ in 1..10 {

        println!("{}", n1);
        next_term = n1 + n2;
        n1 = n2;
        n2 = next_term;


    };
}
