use std::io;
fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    
    println!("Please enter your age: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let age:i32 = input1.trim().parse().expect("Not a valid number");
    
if age <18 {
        println!("This person is too young to have a job here");
        return;
    };
    
    println!("Please state your years of experience: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let experience:i32 = input2.trim().parse().expect("Not a valid number");
    
    let incentive = if experience >= 5 {
        if age >= 40 {
            1_560_000
        } else if age >= 30 {
            1_480_000
        } else if age < 28 {
            1_300_000
        } else {
            0
        }
    } else {
      100_000
    };
println!("The employee's annual incentive is: N{}", incentive);
}