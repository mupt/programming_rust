use std::collections::HashMap;

fn main() {



    let mut scores = HashMap::new();

    scores.insert(2, 10);
    scores.insert(1, 50);

    let key = 1;

    let score = scores.get(&key);

    println!("{:?}", score);


    //let v: Vec<i32> = Vec::new();

    //let v = vec![1, 2, 3];

   // let v = vec![100, 32, 57];

   // for i in &v {
   //     println!("{}", i);

   //
   //
   //
   // }
   //

    //let data = "initial contents";

    //let s = data.to_string();


    //let s = "initial_contents".to_string();

    //let s = String::from("initial_contents");

    //let s = String::from("some_string");

    //for c in s.chars() {
    //    println!("{}", c);
    //}

    //for b in s.bytes() {
    //    println!("{}", b);
    //}



//    let mut scores = HashMap::new();
//
//    scores.insert(String::from("Blue"), 10);
//    scores.insert(String::from("Yellow"), 50);
//
//    //Overwriting an existing value in a hashmap
//    scores.entry(String::from("Blue")).or_insert(25);
//
    //for (k, v) in &scores {
    //    println!("{}: {}", k, v);
    //}

    //println!("{:?}", scores);

    exercise_one();

}

//Exercise 1 - List of integers, return the mean, median mode

fn exercise_one() {


    let v = vec![1, 2, 3, 1, 1,];

    let mut sum = 0;
    let mut frequency = HashMap::new();

    for i in &v {

        //gather sum
        sum = sum + i;


        //for each individual element count how many times it happens

        //If it contains the key, add a 1 to the value
        if frequency.contains_key( & i ) {

            //get the value, add it then re-insert

            let current_count = frequency.remove(&i);

            let int = match current_count {
                Some(_) => 1,
                _ => 0,
            };

            println!("current_count = {:?}", current_count);

            frequency.entry(i).or_insert(int + 1);


        //if it doesn't, add the entry
        } else {

            frequency.insert(i, 1);

        }


    }

    //Average
    //println!("{}", sum / v.len());
    println!("{:?}", frequency);
}


