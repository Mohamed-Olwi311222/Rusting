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
    /*----------- SHADOWING-------- */
    let shadow_var = 10;
    println!("Shadow_var is {shadow_var}");
    let shadow_var = 20; // shadowing
    println!("Shadow_var2 is {shadow_var}");
    {
        //local shadowing
        let shadow_var = 30;
        println!("local Shadow_var is {shadow_var}");
    }
    println!("Shadow_var2 is {shadow_var}"); //same as before the local shadowing
}
