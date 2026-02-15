use std::io::{self, Write}; // Подключаем необходимые модули
pub mod parser;

fn main() {
    println!("Imini 0.0.1");

    loop {
        let mut inp = String::new(); // Объявляем переменную для ввода
        print!(">>: ");
        io::stdout().flush().unwrap(); // Чтобы вывод сразу появился
        io::stdin().read_line(&mut inp).unwrap(); // Читаем строку с ввода

        parser::parse_code_and_execute(&inp); // Передаём ссылку на строку в функцию
    }
}
