fn main() {
    let v = vec![20,40,60,80];
    // v own ths obj in heap

    let v2 = v;
    let v2_return = display(v2);
    println!("In main {:?}",v2_return);


}

fn display(v2_return:Vec<i32>)->Vec<i32>{
    //returning same vector 
    println!("inside display {:?}",v2_return);
    return v2_return;

}