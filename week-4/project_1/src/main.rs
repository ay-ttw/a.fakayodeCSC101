use std::io;
fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the value of a");
    io::stdin().read_line(&mut input1).expect ("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the value of b");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter the value of c");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let d:f32 = (b*b)-4.0*a*c;
    let e:f32 = 0.5;
    let f:f32 = d.powf(e);
    println!("The discriminant is {}", d);                                        

    if d>0.0{
        let x1:f32 = (-b + f) / (2.0 * a);
        println!("The first root of your equation is {}", x1);
        let x2:f32 = (-b + f ) / (2.0 * a);
        println!("The second root of your equation is {}", x2);
        println!("The discriminant is positive and there are two distinct roots");
    }
    
    if d==0.0{
        let x1:f32 = (-b + f) / (2.0 * a);
        println!("The root of your equation is {}", x1);
        println!("The discriminant is zero and there is on one real root");
    }

    if d<0.0{
        println!("The discriminant is negative and there a no real roots");
        println!("The equation has no roots");
    }

}
