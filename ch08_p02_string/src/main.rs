fn main() {

    // The String type, provided by standard library
    // is a growable, mutable, owned, UTF-8 string type.

    // let mut s = String::new();

    let data = "initial contents";
    let mut _s = data.to_string();

    // can also be done as let mut s = "initial contents".to_string();

    // other method
    let _s = String::from("initial contents");

    // appending to a string
    let mut s = String::from("foo");    // s = foo
    s.push_str("bar");                  // s = foobar

    s.push('l'); // pushes a single character to the String

    // concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let _s3 = s1 + &s2;

    // s1 has been moved and can no longer be used
    // we use a reference to s2 because this concatenation
    // uses the function add(self, s:&str) -> String {...}
    // s2 is still a valid string after the operation

    // format macro
    // this call doesn't take ownership of any of its parameters
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let _s = format!("{s1}-{s2}-{s3}");

    // we cannot access single characters from String by indexing.

}
