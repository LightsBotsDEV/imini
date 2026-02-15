use std::io::{self, Write}; // Подключаем необходимые модули

pub fn parse_code_and_execute(code: &str) {
    let codelines = code.trim().lines();  // Разбиваем код на строки
    for line in codelines {
        match line {
            _ if line.starts_with("printmsg ->") => {
                if let Some(message) = line.strip_prefix("printmsg ->") {
                    // Печатаем в консоль
                    println!("{}", message);
                }
            },
            _ if line.starts_with("input") => {
                if let Some(prompt) = line.strip_prefix("input") {
                    let mut input_data = String::new();
                    print!("{}", prompt);  // Выводим подсказку для ввода
                    io::stdout().flush().unwrap(); // Ожидаем, чтобы сообщение вывелось до ввода
                    io::stdin().read_line(&mut input_data).unwrap(); // Читаем строку с ввода
                }
            },
            _ => eprintln!("Name {} is not defined!",line)
        }
    }
}
