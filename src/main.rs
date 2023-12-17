fn main(){
    // let sum = add(1, 2);
    println!("{}", add(20, 45));
}

fn add(a: i32, b: i32) -> i32 {
    let sum = a + b;
    sum
}