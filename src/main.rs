use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Игра: Угадай число!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("Введите число:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Ошибка ввода!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Пожайлуста, введите число!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Слишком маленькое!"),
            Ordering::Greater => println!("Слишком большое!"),
            Ordering::Equal => {
                println!("Угадал!");
                break;
            }
        }
    }
}
