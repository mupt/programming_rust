fn main() {
    enum IpAddrKind {
            V4,
            V6

    }

//    let four = IpAddrKind::V4;
//    let six = IpAddrKind::V6;
//
//    fn route(ip_kind: IpAddrKind) {};
//
//    route(IpAddrKind::V4);
//    route(IpAddrKind::V6);
//
//
//    enum Option<T> {
//        Some(T),
//        None,
//    }
//

    // page 104
   // enum Coin {
   //     Penny,
   //     Nickel,
   //     Dime,
   //     Quarter,

   // }

   // fn value_in_cents(coin: Coin) -> u8 {
   //     match coin {
   //         Coin::Penny     => {
   //             println!("Lucky Penny");
   //             1
   //         },
   //         Coin::Nickel    => 5,
   //         Coin::Dime     => 10,
   //         Coin::Quarter     => 25,
   //     }
   // }

    //Page 105

    #[derive(Debug)]

    enum UsState {
        Alabama,
        Alaska,
   }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
   }

}


