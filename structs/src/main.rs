struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u32
}

// impl User {
//     fn calculate_count(&self) -> u32 {
//         if self.sign_in_count > 0 {
//             self.sign_in_count+=1
//         } else {
//             self.sign_in_count = 0
//         }
//     }
// }

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual; //unit-like struct?

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("test_user"),
        email: String::from("test_user@domain.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("test_user1@domain.com");
    print_user(&user1);

    user1 = build_user(String::from("test"), String::from("test@domain.com"));
    print_user(&user1);

    let user2 = User {
        email: String::from("user2@domain.com"),
        ..user1
    };
    print_user(&user2);
    // print_user(&user1); can't do that cause username was moved from user1 to user2

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    let _subject = AlwaysEqual;
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
        // sign_in_count: {if this.sign_in_count > 0 {this::sign_in_count+=1} else {1}},
    }
}

fn print_user(user: &User) {
    println!(
        "User:\n    username: {},\n    email: {},\n    active: {},\n    sign_in_count: {}\n",
        user.username, user.email, user.active, user.sign_in_count
    );
}
