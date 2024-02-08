struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    
    /* --------------- STRUCTS --------------- */

    let mut myuser = User {
        active: true,
        username: String::from("liamdugg"),
        email: String::from("liamdug@gmail.com"),
        sign_in_count: 1,
    };

    // either the full struct instance is mutable
    // or none of its elements is

    myuser.email = String::from("liamdugg@gmail.com");

    let usr = build_user(String::from("email@gmail.com"), String::from("user"));
    println!("{}\n{}\n{}\n{}", usr.active, usr.username, usr.email, usr.sign_in_count);

    // we can set a struct field to a value we want and then tell rust
    // we want "usr2" to copy all the other fields from "usr"
    let usr2 = User {
        email: String::from("liamdug@hotmail.com"),
        ..usr
    };

    println!("{}\n{}\n{}\n{}", usr2.active, usr2.username, usr2.email, usr2.sign_in_count);

    // because we copied the username field from usr to usr2 this field is no longer
    // valid (or owned) by usr because it is of type String.

    /* --------------- TUPLE STRUCTS --------------- */

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let yellow = Color(255, 255, 0);
    let origin = Point(0,0,0);

    println!("{} {} {}", yellow.0, yellow.1, yellow.2);
    println!("{} {} {}", origin.0, origin.1, origin.2);

    // even though both types are made up of three i32 values,
    // each struct is its own type. if we try to pass a Color struct
    // to a function that requires a Point struct it wont work.

    // Unit-Like structs
    // struct AlwaysEqual;
    
    // can be useful when you need to implement a trait on some type but 
    // don’t have any data that you want to store in the type itself.
    // let subject = AlwaysEqual;

}

fn build_user(email: String, username: String) -> User {
    User {
        active : true,
        username,
        email,
        sign_in_count: 1
    }
}