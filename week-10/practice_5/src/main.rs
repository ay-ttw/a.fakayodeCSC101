fn main() {
    //a list of numbs
    let x = vec![100,200,300];
    borrow_vector(&x); //pass by ref

    println!("Printing the values from main() x[0]={}",x[0]);
    println!("*****************************");
}

fn borrow_vector(z:&Vec<i32>){
    println!("****************************");
    println!("Inside print_vector function {:?} \n",z);
    println!("-----------------------------");
}