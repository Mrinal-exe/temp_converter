use std::io;
fn main() {
    loop {
        println!("----------TEMPERATURE CONVERTER----------");

        println!("Select the operation you want to perform.");
        println!("1. Convert Celsius to Fahrenheit.");
        println!("2. Convert Fahrenheit to Celsius.");
        println!("3. Quit!");

        let mut input = String::new();

        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read message!!");
        
        let input:i32 = match input.trim().parse(){
            Ok(number) => number,
            Err(_) => continue,
        };

        if input == 1{
            celsius_to_fahren();
        }else if input == 2 {
            fahren_to_celsius();
        }else if  input == 3{
            break;
        }else {
            println!("PLEASE PROVIDE A VALID INPUT!!")
        };
    }
}

fn celsius_to_fahren(){
    let mut x = String::new();
    println!("Please enter the tempurature in Celsius.");
    io::stdin().read_line(&mut x).expect("Failed to read message");

    let x: i32 = x.trim().parse().expect("Failed to read ");

    let a = (x*9)/5;
    println!("{x} C = {} F", a+32);
}

fn fahren_to_celsius(){
    let mut x = String::new();
    println!("Please enter the tempurature in Fahrenheit.");
    io::stdin().read_line(&mut x).expect("Failed to read message");

    let x: i32 = x.trim().parse().expect("Failed to read ");

    let a = ((x-32)*5)/9;
    println!("{x}F = {a}C");
}