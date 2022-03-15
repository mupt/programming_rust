#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {

    let rect1 = Rectangle { width: 30, height: 50 };

    println!("area is {:#?}", rect1);
}

//fn area(rectangle: &Rectangle) -> u32 {
//    rectangle.width * rectangle.height
//}
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


    //let rect1 = (30, 50);

    //println!(
    //    "The area of the rect is {} square pixels.",
    //    area(rect1)
    //);


//}

//fn area(dimensions: (u32, u32)) -> u32 {
//    dimensions.0 * dimensions.1
//}
