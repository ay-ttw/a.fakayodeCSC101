fn main() {
    let v = vec![10,20,30];
    
    let v2 = v.clone(); //to move ownership ;

    display(v2);
    //v2 is moved to display and is invalidated

    println!("In main {:?}",v);
    //v2 is no longer usable

}
  fn display(v:Vec<i32>){
       println!("inside display {:?}",v);
    }