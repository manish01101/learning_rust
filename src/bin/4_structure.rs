struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
#[derive(Debug)] // debug trait
struct Rectangle {
    width: u32,
    height: u32,
}
/* METHOD & ASSOCIATED FUNCTION*/
impl Rectangle {
    // METHOD works on the instance of stuct/enum/trait object
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // getter
    fn width(&self) -> bool {
        self.width > 0
    }
    // ASSOCIATED FUNCTION
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
/* MULTIPLE IMPL BLOCKS */
// “There’s no reason to separate these methods into multiple impl blocks here, but this is valid syntax”
impl Rectangle {
    // takes arg as another Rectangle instance
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

/* MAIN FN */
fn main() {
    let user1 = User {
        active: true,
        username: String::from("manish"),
        email: String::from("manish@gmail.com"),
        sign_in_count: 2,
    };
    println!("user1 username: {}", user1.username);
    // creating instances from other instances
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("a@gmail.com"),
        sign_in_count: user1.sign_in_count,
    };
    // .. => struct update syntax to set rest of values from user2
    let _user3 = User {
        email: user2.email,
        ..user2
    };

    // tuple struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    // unit struct
    struct AlwaysEqual;
    let _subject = AlwaysEqual;

    let rect1 = Rectangle {
        width: 30,
        height: 40,
    };
    println!("rect1 is {:?}", rect1); // println! takes only reference
    println!("rect1 is {:#?}", rect1);
    // PRINTING USING dbg!
    // dbg!(rect1); // dbg! takes ownership and after process returns ownership
    dbg!(&rect1); // passing as reference so not take ownership
    println!("area is {}", area(&rect1)); // area function
    println!("area is {}", rect1.area()); // area method

    let rect2 = Rectangle {
        width: 10,
        height: 70,
    };
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));

    // CALLING ASSOCIATED FUNCTION
    let square = Rectangle::square(2);
}

/* USER FUNCTION */
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // parameter and User's field has same name
        email,
        sign_in_count: 2,
    }
}
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
