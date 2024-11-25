use std::io;

fn trapezium(){
    let mut input1 =String::new();
    let mut input2 =String::new();
    let mut input3 =String::new();

    println!("\nArea of Trapezium formula = (height/2)x(base1+base2)");


    println!("\nPlease state the value of the height");
    io::stdin().read_line(&mut input1).expect("Not a valid input");
    let height:f32 = input1.trim().parse().expect("Not a valid input");

    println!("\nPlease state the value of base1");
    io::stdin().read_line(&mut input2).expect("Not a valid input");
    let base1:f32 = input2.trim().parse().expect("Not a valid input");

    println!("\nPlease state the value of base2");
    io::stdin().read_line(&mut input3).expect("Not a valid input");
    let base2:f32 = input3.trim().parse().expect("Not a valid input");

    let area:f32 = height/2.0*(base1+base2);
    println!("The area of the Trapezium is: {}",area);
}

fn rhombus(){
    let mut input1 =String::new();
    let mut input2 =String::new();

    println!("\nArea of Rhombus formula = 1/2 x Diagonal 1 x Diagonal 2"); 


    println!("\nPlease state the value of Diagonal 1");
    io::stdin().read_line(&mut input1).expect("Not a valid input");
    let diagonal1:f32 = input1.trim().parse().expect("Not a valid input");

    println!("\nPlease state the value of Diagonal 2");
    io::stdin().read_line(&mut input2).expect("Not a valid input");
    let diagonal2:f32 = input2.trim().parse().expect("Not a valid input");

    let area:f32 = 0.5*diagonal1*diagonal2;
    println!("\nThe area of the Trapezium is: {}",area);
}

fn parallelogram(){
    let mut input1 =String::new();
    let mut input2 =String::new();

    println!("\nArea of Parallelogram formula = base x altitude");


    println!("\nPlease state the value of the base");
    io::stdin().read_line(&mut input1).expect("Not a valid input");
    let base:f32 = input1.trim().parse().expect("Not a valid input");

    println!("\nPlease state the value of the Perpendicular height(altitude)");
    io::stdin().read_line(&mut input2).expect("Not a valid input");
    let altitude:f32 = input2.trim().parse().expect("Not a valid input");

    let area:f32 = base*altitude;
    println!("\nThe area of the Trapezium is: {}",area);
}

fn cube(){
    let mut input1 =String::new(); 

    println!("\nArea of Cube formula = 6 x (length of side)^2");

    println!("\nPlease state the value of the length of the side of the cube");
    io::stdin().read_line(&mut input1).expect("Not a valid input");
    let length:f32 = input1.trim().parse().expect("Not a valid input");

    let area:f32 = 6.0*length.powf(2.0);
    println!("\nThe area of the Trapezium is: {}",area);
}

fn cylinder(){
    let mut input1 =String::new();
    let mut input2 =String::new();

    println!("\nVolume of Cylinder formula = π*radius^2 *height");


    println!("\nPlease state the value of the radius");
    io::stdin().read_line(&mut input1).expect("Not a valid input");
    let radius:f32 = input1.trim().parse().expect("Not a valid input");

    println!("\nPlease state the value of the height");
    io::stdin().read_line(&mut input2).expect("Not a valid input");
    let height:f32 = input2.trim().parse().expect("Not a valid input");

    let pi:f32 = 3.14159;

    let area:f32 = pi*radius.powf(2.0)*height;
    println!("\nThe area of the Trapezium is: {}",area);
}

fn main() {
    let mut input = String::new();
    

println!("\nHello! what would you like to calculate today? (Select the number of the calculation preffered)
1 Area of a Trapezium 
2 Area of a Rhombus
3 Area of a Parallelogram
4 Area of a Cube
5 Volume of a Cylinder ");
io::stdin().read_line(&mut input).expect("Not a valid input");
let option:i32 = input.trim().parse().expect("Not a valid input");

if option == 1{
     trapezium()
}

if option == 2{
    rhombus()
}

if option == 3{
    parallelogram()
}

if option == 4{
    cube()
}
if option == 5{
    cylinder()
}

if option > 5{
    println!("This option is not valid");
    return;
}

}
