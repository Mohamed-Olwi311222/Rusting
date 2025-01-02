enum IpAddrKind{
    V4,
    V6
}
//methods of @IpAddrKind enum
impl IpAddrKind {
   fn print_v4(&self){} 
}
//enum with different types of fields
enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String)
}

//enum inside a struct
struct IpAddr{
    kind: IpAddrKind,
    address: String
}
//option enum is available in the standard library
/*
enum Option<T>{
    None,
    Some(T)
}
*/
fn main() {
    //assignment
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("192.168.1.1")
    };

    let home2 = IpAddress::V4(192, 168, 1, 1);
    
    four.print_v4();    //calling of enum method

    //option enum
    let some_number = Some(5);
    /*
    For absent_number , Rust requires us to annotate the overall Option type: 
    the compiler can’t infer the type that the corresponding Some variant will hold by looking only at a None value.
     */
    let absent_number: Option<i32> = None;//

}
