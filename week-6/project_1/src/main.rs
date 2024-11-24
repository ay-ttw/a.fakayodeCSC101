use std::io;
fn main() {
   let mut input1 = String::new();
   let mut input2 = String::new();

   println!("Menu			      Price
P = Poundo Yam/Edinkaiko Soup 	   - N3,200
F = Fried Rice & Chicken 		- N3,000
A = Amala & Ewedu Soup			- N2,500
E = Eba & Egusi Soup			- N2,000
W = White Rice & Stew			- N2,500 ");
    
    println!("Please state the item(s) of your choice (P,F,A,E or W)");
    io::stdin().read_line(&mut input1) .expect("Failed to read input");
    let x = input1.trim();

    println!("How many portions?");
    io::stdin().read_line(&mut input2) .expect("Failed to read input");
    let y:f32 = input2.trim().parse().expect("Not a valid string");
    
    let P:f32 = 3_200.00;
    let F:f32 = 3_000.00;
    let A:f32 = 2_500.00;
    let E:f32 = 2_000.00;
    let W:f32 = 2_500.00;
    
    let mut price:f32 = P*y;

    if x == "P"{
        let price = P*y;
    };

    if x == "F"{
        let price = F*y;
    };

    if x == "E"{
        let price = E*y;
    };

    if x == "A"{
        let price = A*y;
    };

    if x == "W"{
        let price = W*y;
    };

    if price > 10_000.00{
        let discount = price - (price * (5.0/100.0));
        println!("The Old price of your order is {}", price);
        println!("The New price of your order is {}", discount);
    };
        if price <10_000.00{
            println!("The price of your order is {}", price);
        };
    
}