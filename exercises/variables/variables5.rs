// variables5.rs
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a hint.


fn main() {
    let number:&str = "T-H-R-E-E"; // don't rename this variable
    t1(number);
    println!("Spell a Number : {}", number);
}


fn t1 (number:&str) {
    println!("Spell a Number : {}", number);
    let number = 3; // don't rename this variable
    
    {
        let number = number * 2;
        println!("Number plus two is : {}", number + 2);
    }
    println!("Number plus two is : {}", number );
}
