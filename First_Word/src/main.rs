fn first_word(s : &str)-> &str{  //return string slice and accept string slice as it is same as string reference
    let bytes = s.as_bytes(); //convert s to an array of bytes
    for (i,&chr) in bytes.iter().enumerate() {
        if chr == b' '{
            return &s[..i];
        }
    }
    return &s[..];
}
fn main() {
    let s1 = String::from("Hello World");
    let first_word = first_word(&s1);
    println!("first word of {s1} is {first_word}");
    // s1.clear();  //error 
    //Rust prevents mutable and immutable borrows from coexisting to ensure safety. 
    //If s.clear() were allowed while word held an immutable reference,
    // word could end up pointing to invalid memory, causing undefined behavior
}
