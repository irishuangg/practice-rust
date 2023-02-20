// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     let user1 = User {
//         active: true,
//         username: String::from("someusername123"),
//         email: String::from("someone@example.com"),
//         sign_in_count: 1,
//     };

//     let user2 = User {
//         username: String::from("someusername456"),
//         email: String::from("another@example.com"),
//         ..user1
//     };

//     println!("user1's active now looks like {}, and user2's active now looks like {}", user1.active, user2.active);
//     println!("user1's email has {}, and user2's email now has {}", user1.email, user2.email);
//     // println!("user1's username has {}, and user2's username now has {}", user1.username, user2.username); <<< if this line has not been implemented, `user1.username` will show a borrowing a moved value error
// }

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("first element of black is {}", black.0);
}