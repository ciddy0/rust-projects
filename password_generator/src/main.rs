use rand::Rng;
use std::env;

fn main() {
    // get password length via command line arguments
    let args: Vec<String> = env::args().collect();
    let password_length = &args[1];
    let int_password_len: i32 = password_length.parse().unwrap();
    let is_nums = &args[2];
    let is_special = &args[3];
    println!("Password length: {password_length}");

    let mut alpha = String::from("qwertyuiopasdfghjklzxcvbnm");
    let numbers = "1234567890";
    let special_chars = "!@#$%^&*()";

    if is_nums == "1" {
        alpha.push_str(numbers);
    }
    if is_special == "1" {
        alpha.push_str(special_chars);
    }

    let mut password = String::from("");
    for i in 0..int_password_len {
        let secret_number = rand::thread_rng().gen_range(0..=alpha.len() - 1);
        let char = alpha.chars().nth(secret_number);
        password.push(char.unwrap());
    }
    println!("{password}")
}
