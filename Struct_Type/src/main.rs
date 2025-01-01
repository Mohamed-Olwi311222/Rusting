use std::hash::BuildHasherDefault;

struct User{
    active: bool,
    user_name: String,
    email: String,
    sign_in_count: u64
}
//tuple struct
struct color(i32, i32, i32);
fn main() {
    let user1 = User{
        active: true,
        user_name: String::from("Mohamed_Olwi"),
        email: String::from("mohamedolwi38@gmail.com"),
        sign_in_count: 10
    };
    //to access struct fields, we use dot notation
    println!("{}",user1.user_name);
    // user1.sign_in_count = 20;    //illegal as user1 is immutable
    let mut user2 = User{
        active: true,
        user_name: String::from("AceMido"),
        email: String::from("acemido2004@gmail.com"),
        sign_in_count: 20
    };
    user2.sign_in_count = 20;
    println!("{}", user2.sign_in_count);
    let email = String::from("mohamedolwi3112@gmail.com");
    let user_name = String::from("AceNotMido");
    let user3 = build_user(email.clone(), user_name);   //notice that build_user take the ownership of user_name
    println!("{}", email);
    println!("user3 sigin in count is {}", user3.sign_in_count);
    // println!("{}", user_name); //illegal as build_user took the ownership of it

    /*Creating Instances from Other Instances with Struct Update Syntax */
    let user4 = User{
        active: user3.active,
        user_name: user3.user_name,         //user3,user_name ownership moved here
        email: String::from("anotheremail@gmail.com"),
        sign_in_count: 10
    };
    // println!("{}", user3.user_name); illegal as user4.user_name took its ownership
    let user5 = User{
        email:String::from("anotheremail2@gmail.com"),
        ..user2     //user5 took the owenership of the rest of the struct of user4
    };
    // print!("{}", user2.user_name); illeagl as user5 took its ownership

    /*---------tuple struct---------- */
    let black = color(0, 0 ,0);
    println!("{}", black.0);
}
fn build_user(email: String, user_name: String)-> User{
    User {
        active: true,
        user_name,  //because the struct and the parameters has the same name so instead of we type username: user_name, we can do this
        email,
        sign_in_count: 10
    }
}
