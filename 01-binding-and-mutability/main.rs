//fix the error below with the least amount of modification to the code
fn main() {
    let x: i32 = 5; //Uninitialized but used
    let y: i32;     //Uninitialized but also unused, only a warning!
    assert_eq!(x,5);
    println!("Success!");
}
