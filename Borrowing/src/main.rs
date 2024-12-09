fn main() {
    let str1 = String::from("Hello");
    let str2 = str1;
    // println!("{str1} world");        //invalid variable as str1 is dropped after str2 has been assigned to it
                                        //str1 has been moved to str2 not shallow copied
    println!("{str2} world");
    take_ownership(str2); //str2 is no more valid after this point as the function took its ownership
    // println!("{str2} world");      //will throw compile error
    
    let x = 1;
    let y = x;   // value of x copied to y
    make_copy(y); //y is copied not moved to some_integer so its perfectly valid to use y after this point
    println!("x = {x}, y = {y}");
    let (str3 , length)= gives_ownership(); //str3 took the owership of some_string
    println!("{str3} length is {length}");
    let str4 = takes_and_gives_back(str3); //str3 is no valid after this point
                                                                //str3 has been moved to str4
    // println!("{str3}");           //invalid
    println!("{str4}");
}
fn takes_and_gives_back(some_string : String) -> String{
    some_string
}
fn gives_ownership() -> (String, usize){
    let some_string = String::from("Yours");
    let length = some_string.len();
    (some_string, length)
}
fn take_ownership(some_string : String){
    println!("take_ownership has been called");
    println!("{some_string}");
}//drop is called and some_string is freed and goes out of scope
fn make_copy(some_integer: u32){
    println!("make_copy has been called");
    println!("{some_integer}");
}
