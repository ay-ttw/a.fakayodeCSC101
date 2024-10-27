fn main(){
    let p: f64 = 210_000.0;
    let r: f64 = 5.00;
    let n: i32 = 3;
    let a= p*(1.0+(r/100.0)).powi(n); //formula for amount
    println!("Amount is {}",a);
    let ci= a - p; //formula for depreciation
    println!("The new price of the car is {}",ci);
    
}