//define the dimentsions of a rectangle
struct Laptops{
    hp:f64,
    ibm:f64,
    toshiba:f64,
    dell:f64,

}

//logic to calculate the area of a rectangle
impl Laptops{
    fn cost (&self)->f64{
        //use the . operator to fetch the value of a field via the self keyword
        self.hp * 3.0 + self.ibm * 3.0 + self.toshiba * 3.0 +self.dell * 3.0 
    
    }
}

fn main() {
    //instanatiate the stricture
    let price = Laptops {
        hp:650_000.0,
        ibm:755_000.0,
        toshiba:550_000.0,
        dell:850_000.0,
    };
    //print the rectangle's area
    println!("As the customer orders 3 from each brand \nThe total cost comes out to {} ", price.cost());
}


