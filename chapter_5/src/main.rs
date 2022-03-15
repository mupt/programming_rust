fn main() {

    //Structs Page 83
    //
//
//    struct User {
//        username: String,
//        email: String,
//        sign_in_count: u64,
//        active: bool,
//    }
//
//    let user1 = User {
//        email: String::from("someone@example.com"),
//        username: String::from("someusername123"),
//        active: true,
//        sign_in_count: 1,
//    };
//
//    //Tuple Structs look like this
//    struct Color(i32, i32, i32);
//
//    let black = Color(0,0,0);


    let rect1 = (30, 50);

    println!(
        "The area of the rect is {} square pixels.",
        area(rect1)
    );

}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
