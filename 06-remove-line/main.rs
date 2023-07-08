fn main(){
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and rebinding
    // variables in rust are inmutable
    x += 3;

    let y: i32 = 4;
    let y = "I can also ve bound to text!";

    println!("Success!");
}
