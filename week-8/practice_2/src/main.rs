fn main() {
    let v = vec!["C","O","M","P","U","T","E","R"];

    let mut input1  = String::new();

    println!("Enter and index value between (0-7)");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    // index = length minus 1
    let index:usize = input1.trim().parse().expect("Invalid input");

    let ch = v[index];

    print!("{} is the character for index [{}]\n",ch,index);
}
