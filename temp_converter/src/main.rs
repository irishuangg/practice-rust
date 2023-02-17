use ::std::io::stdin;

fn main(){
    println!("Input the temperature and type");
    
    let mut num = String::new();
    let mut system = String::new();

    stdin()
        .read_line(&mut num)
        .expect("Failed to read line");
    
    stdin()
        .read_line(&mut system)
        .expect("Failed to read line");
    
    let num: i32 = num.trim().parse().expect("Please type a number!");
    

    // this .trim() thing is really annoying... it's because the program sees "F/n" instead of "F"
    if system.trim() == "F" {
        let num = (num - 32) * 5 / 9;
        println!("That's {num} degrees Celsius");
    } else {
        let num = num / 5 * 9 + 32;
        println!("That's {num} degrees Fahrenheit");
    }
}