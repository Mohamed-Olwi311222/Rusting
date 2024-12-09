fn main() {
    let s1 = String::from("Hello");
    let length = calculate_length(&s1); //we send a reference to calculate_length
                                                           //so its perfectly valid to use s1 as it didnt go out of scope
    println!("{s1} length is {length}");

    let mut s2 = String::from("Hello, World");
    let s2_length = calculate_length_mut(&mut s2);
    println!("{s2} length is {s2_length}");

    let _r1 = &mut s2;
    let _r2 = &mut s2;
    // println!("{}, {}", _r1, _r2); //will result in compile error

    //another reference rule :)
    let mut s2 = String::from("Embedded");
    let _r4 = &s2;
    let _r5 = &s2;
    // let _r6 = &mut s2;   //wont work as Users of an immutable reference don’t expect the value to suddenly change out from under them
    println!("{_r4} and {_r5}");
    let _r6 = &mut s2;  //will work as _r4 and _r5 goes out of scope
    println!("{_r6}");

    // let reference_to_nothing = dangle();     //wont work as the some_string is owned by another function in stack
    // println!("{reference_to_nothing}");
}
/*
fn dangle() -> &String{
    let some_string = String::from("I am a dangling ptr");
    &some_string
}//some_string will go out of scope and drop will be called
*/
fn calculate_length_mut(some_string: &mut String)->usize{
    some_string.push_str("!!!!");//some_string has mutable reference so it will work
    some_string.len()
}
fn calculate_length(some_string: &String) -> usize{
    // some_string.push(", World"); 
    //wont work as some_string Just as variables are immutable by default, 
    //so are references. We’re not allowed to modify 
    // something we have a reference to. 
    some_string.len()
}//here some_string will go out of scope but because it doesnt have ownership
//of what it refers to, it isnt dropped