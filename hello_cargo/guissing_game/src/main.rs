use std::io;
use rand::Rng;

fn main(){
    println!("Игра угадай число");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    println!("The secret number is: {secret_number}");
    
    println!("Введите число");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Ошибка чтения");

    println!("Ваше число: {}", guess)

    
}