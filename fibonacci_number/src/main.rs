fn main() {
    let mut n = 1;
    loop {
        let result = n + (n + 1);
        println!("The fibonacci number is {result}.");
        
        if n == 100 {
            break;
        }

        n += 1;
    }
}
