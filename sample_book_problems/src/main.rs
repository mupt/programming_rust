fn main() {


    //First Word String Finder
    let s = String::from("hellosss eggs");
    let word = first_word(&s);
    println!("Your Length Is {}", word);


    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];

    println!("{}", slice);
}

fn first_word(s: &String) -> &str {

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }

    }

    &s[..]
}

