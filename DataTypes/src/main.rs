fn main() {
    //tuple
    let mut tup :(i8, f32 , char) = (1, 4.1212, 'S');
    println!("tup.0 = {}", tup.0);
    tup.1 = 3.14;                       //change the second element in tuple tup
    println!("tup.1 = {}", tup.1);
    println!("tup.2 = {}", tup.2);

    //array
    let mut months = ["January", "February", 
                                "March", "April", "May", 
                                "June", "July", "August", 
                                "September", "October", "November", "December"];
    months[0] = "JAN";
    let first = months[0];
    println!("first month is {}", first);
    let multi = [3; 5];                 //an array with 5 elements with value 3 repeated in them
    println!("multi[4] = {}", multi[4]);
    //expression and statements
    let x = 5 + 3;  //5 + 3 is an expression
    let statement= {
        let expression = 3;
        expression+1
    };
    println!("x is {x}");
    println!("statement variable is {}", statement);
}
