fn main() {

    {
        // s valid only inside this scope
        // string literal, immutable goes to stack
        let s = "hello world";
        println!("{s}");
    }

    // String type is for data on the heap
    // here we make a String from a string literal
    let mut s = String::from("hello"); 
    println!("{s}");

    // mutable, goes to heap
    s.push_str(", world!");
    println!("{s}");

    // with push_str we asked for more memory implicitly
    // in rust we dont need to "free" it as in C.
    // memory is automatically returned (freed) once the variable goes out of scope.

    {   
        // ask for memory
        let s = String::from("hello");
        println!("s --> {s}");
        // scope end, rust calls drop to free the memory of this scope
    }

    /* ---------------------------------------------------------- */

    // let x = 5;
    // let y = x;
    // we cannot do the same with Strings
    // strings have 3 "parts" stored in the stack
    //  - pointer (to the heap)
    //  - length
    //  - capacity
    let s1 = String::from("hello");
    
    // when we do this we are copying the three values in the stack
    // both s1 and s2 access the same memory address in the heap (they have the same pointer).
    let s2 = s1;
    println!("s2 --> {s2}");

    // when the scope of s1 and s2 ends, drop will be called on both of them causing a problem.
    // to fix this, when we run the "let s2 = s1;" line, s1 is considered no longer valid.

    // if we DO want to copy the data of the heap instead of the one in the stack we use clone.
    let mut s3 = String::from("s3");
    let s4 = s3.clone();

    // the function will take ownership of s4 as it is a String
    take_ownership(s4);
    // s4 is no longer valid here as it was owned by the function and then dropped.

    // if we did the same with an integer this wouldnt have happened because the are
    // of known size and therefore stored in the stack, so the function wouldnt have taken
    // ownership. Instead, it would have made a copy of the variable to work with.

    // if we wish to recover ownership from a function we would have to return the value.
    s3 = take_and_return_ownership(s3);
    println!("{s3} --> from main");

    // what if we want to still have ownership of the variable 
    // but without having to return the same variable from the function
    // we use references.
    let size = calculate_length(&s3);
    println!("size -> {size}");

    // we cannot modify a value that has been borrowed.
    // that is because references are immutable by default too.
    // we must use a mutable reference in order to do that.
    let mut s5 = String::from("hello");
    change_string(&mut s5);

    // if a value has a mutable reference, it cannot have any other references to it.
    let mut s = String::from("hello");
    
    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems
    
    let r2 = &mut s;

    // a reference scope start from where it is introduces until the last time its used.
    let mut s = String::from("hello");
    
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    
    println!("{r1} and {r2}");
    
    // Variables r1 and r2 will not be used after this point.
    let r3 = &mut s; // no problem
    println!("{r3}");

    /* ---------------------------------------------------------- */

    // a string slice is a reference to part of a String
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    // this does not create other variables in the heap
    // as slices are still references
    // world would have a pointer to the index 6 of s.
    // slices have only two "parts" in the stack, a pointer and length
    // there is no capacity element.

}

fn change_string(s: &mut String){
    s.push_str(", world!");
}

fn calculate_length(s: &String) -> usize {
    // we pass a reference to s
    // in this case drop wont be called
    // and we will still own the string passed on function return.
    // we call this referencing "borrowing".
    s.len()
}

fn take_ownership(s: String){
    println!("{s}");
}

fn take_and_return_ownership(s: String) -> String {
    println!("{s} --> from the function");
    s
}
