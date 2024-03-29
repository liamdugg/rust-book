use std::vec;

fn main() {

    let _v: Vec<i32> = Vec::new();

    // vec! macro creates new vector that holds values you give to it
    // infers the type the vector will store
    let _vm = vec![1, 2, 3];
    
    let mut v = Vec::new();

    // add elements to vector
    v.push(5);
    v.push(2);

    
    let v = vec![1,2,3,4,5];

    // get element by indexing
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // get element with get method.
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    // iterate over immutable vector
    let v = vec![100, 32, 57];
    
    for i in &v {
        println!("{i}");
    }

    // iterate over mutable vector
    let mut v = vec![100, 32, 54];

    for i in &mut v {
        *i += 50;
    }

    // vector of enum to store different types of data
    // using a single enum type
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // A vector is freed when it goes out of scope
    // as well as all of its contents

    {
        let _v = vec![1,2,3,4];
        // do something

    } // goes out of scope, gets dropped



}
