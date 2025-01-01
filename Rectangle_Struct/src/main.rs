#[derive(Debug)]
struct Rectangle{
    height: u32,
    width: u32
}
impl Rectangle{
    fn area(&self)-> u32{
        self.width * self.height
    }
    fn width(&self)-> bool{
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle)-> bool{
        self.width > other.width && self.height > other.height
    }
    //associated function
    fn square(size: u32)-> Self{
        Self { 
            height: size, 
            width: size 
        }
    }
}
fn main() {
    let rec1 = Rectangle{
        height: 10,
        width: 5
    };
    println!("The area of rectangle {rec1:?} is {}", area_of_rectangle(&rec1));


    //using dbg! instead of println!
    let scale = 30;
    let rec2 = Rectangle{
        height: dbg!(scale * 2),
        width: 30
    };
    dbg!(&rec2);

    //using Rectangle method
    println!("area of rectangle rec2 is {}",rec2.area());

    if rec2.width(){
        println!("The width of the rectangle is nonzero and it is {}", rec2.width);
    }
    println!("Can rec2 lengthhold rec1 in it? {}", rec2.can_hold(&rec1));
    let squ1 = Rectangle::square(10);
}

fn area_of_rectangle(rec: &Rectangle) ->u32{
    rec.height * rec.width
}