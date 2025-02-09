fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let x = vec[0]; //Avoid mutable borrow
    vec.push(3);
    println!("Value of x: {}", x);
}
//Alternative solution using clone
fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let x = vec[0].clone();
    vec.push(3);
    println!("Value of x: {}", x);
} 