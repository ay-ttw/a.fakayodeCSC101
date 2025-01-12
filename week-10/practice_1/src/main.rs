fn main() {
    
    let v = vec![101,250,330,400];
    // v owns the obj in heap

    //only one var owns the heap memory at a time

    let v2 = v;
    println!("{:?}",v2);
}
