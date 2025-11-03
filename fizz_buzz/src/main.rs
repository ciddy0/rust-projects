fn main() {
    for number in (0..15).rev() {
        if number % 3 == 0 && number % 5 == 0 {
            println!("Fizz buzz!");
        } else if number % 3 == 0 {
            println!("Fizz!");
        } else if number % 5 == 0 {
            println!("Buzz!");
        }
    }
}
