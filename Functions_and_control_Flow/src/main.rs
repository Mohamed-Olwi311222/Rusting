fn array_sum(arr : &[i32]) -> i32 {
    let mut sum: i32 = 0;
    for elem in arr {
        sum += elem;
    }
    sum
}
fn arr_multi(arr1 : [i32; 5], arr2: [i32 ; 5]) -> [i32; 5]{
    let mut res:[i32; 5] = [0; 5];
    for index in (0..4).rev() {
        res[index] = arr1[index] * arr2[index];
    }
    return res;
}
fn print_arr(arr : &[i32])
{
    print!("[");
    for elem in arr{
        print!("{elem}, ");
    }
    println!("]");
}
fn main() {
    let arr1: [i32; 5] = [1, 2, 3, 4 ,5];
    let arr2 : [i32; 5] = [6, 7 ,8 ,9 ,10];
    print!("sum of arr1[i32; 5] = ");
    print_arr(&arr1);
    println!("is {}", array_sum(&arr1));
    let res : [i32; 5] = arr_multi(arr1, arr2);
    print!("arr1 is = ");
    print_arr(&arr1);
    print!("arr2 is = ");
    print_arr(&arr2);
    print!("multiplication of arr1 and arr2 is [");
    for elem in res {
        print!("{elem}, ");
    }
    println!("]");
}
