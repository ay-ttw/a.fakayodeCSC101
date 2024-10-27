fn main(){
    let toshiba = 450_000*2; 
    let mac = 1_500_000*1;
    let hp = 750_000*3;
    let dell = 2_850_000*3;
    let acer = 250_000*1;
    let sum = toshiba+mac+hp+dell+acer;
    println!("Sum is {}", sum);
    let avg = sum/5;
    println!("Average is {}", avg);

}