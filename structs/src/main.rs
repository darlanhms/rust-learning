// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }


// fn main() {
//     let user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("someusername123"),
//         active: true,
//         sign_in_count: 1,
//     };

//     let active_label = if user1.active { "active" } else { "not active" };


//     println!("User {} with email {} is {} and logged in {} times", user1.username, user1.email, active_label, user1.sign_in_count)
// }

/**
 * 
 * Spread operator
 * 
 */
// fn main() {
//     let user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("someusername123"),
//         active: true,
//         sign_in_count: 1,
//     };

//     let user2 = User {
//         email: String::from("another@example.com"),
//         username:  String::from("anotherusername123"),
//         ..user1
//     };

//     let active_label = if user2.active { "active" } else { "not active" };

//     println!("User {} with email {} is {} and logged in {} times", user2.username, user2.email, active_label, user2.sign_in_count)
// }

/**
 * 
 * Example program using rectangle
 * 
 */

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height

    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    
    dbg!(&rect1);
    println!("The rectangle area is {}", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

