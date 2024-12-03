const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
fn main() {
    let immutable_var = 5;
    let mut mutable_var = 2;
    println!("The value of immutable_var is {immutable_var}" );
    println!("The value of mutable_var is {mutable_var}" );
    // immutable_var = 10;                  //will cause an error
    mutable_var = 6;
    println!("The value of immutable_var is {immutable_var}" );
    println!("The value of mutable_var is {mutable_var}" );
    println!("The value of THREE_HOURS_IN_SECONDS is {THREE_HOURS_IN_SECONDS}");
}
