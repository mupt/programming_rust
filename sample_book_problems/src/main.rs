fn main() {



    let s = String::from("hello");

    first_word(&s);
    println!("{}", first_word(&s));
}

fn first_word(s: &String) -> usize {

    return s.len();
}

