fn main() {
    // let user1 = User {
    //     name: String::from("Duffy"),
    //     email: String::from("duffyDev@gmail.com"),
    //     password: String::from("test1234!"),
    //     age: 20,
    //     login_count: 0,
    //     active: true
    // };
    //
    // println!("User name: {}", user1.name);

    let my_rectangle_1 = Rectangle {
        width: 25,
        height: 30
    };

    let my_rectangle_2 = Rectangle {
        width: 20,
        height: 30
    };

    println!("my rectangle area: {}", my_rectangle_1.get_area());
    println!("can rectangle1 hold rec2?: {}", my_rectangle_1.can_hold(&my_rectangle_2));
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn get_area(&self) -> u64 {
        u64::from(self.height * self.width)
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

// struct User {
//     name: String,
//     email: String,
//     password: String,
//     age: u8,
//     login_count: u64,
//     active: bool
// }
