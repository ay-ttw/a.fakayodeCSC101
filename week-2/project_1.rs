fn main(){
    let p: f64 = 520_000_000.0;
    let r: f64 = 10.00;
    let n: i32 = 5;
    let a= p*(1.0+(r/100.00)).powi(n); //formula for amount
    println!("Amount is {}",a);
    let ci= a - p; //formula for compound interest
    println!("The Compound Interest is {}",ci);
    
}