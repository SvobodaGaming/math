use read_input::prelude::*;
use std::io::{stdin, stdout, Read, Write};
use crate::full_square_equations::full_square_equations;

mod full_square_equations;

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Tap 'Enter' for close...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn main() {
    println!("  ");
    println!("svobodagaming_math");
    println!("Версия программы: 0.1.2");
    println!("  ");
    println!("Для начала, выберите нужный режим решения уравнений");
    println!("Доступные на данный момент:");
    println!("  ");
    println!("Полные квадратные уравнения - (1)");
    println!("Неполные квадратные уравнения - (2) (СКОРО)");
    println!("  ");
    let a = input::<u8>().msg("Напишите номер нужного режима решения уравнения: ").err("Введено неверное значение! Используйте цифры написанные выше!").get();
    if a == 1 {
        full_square_equations();
    } else if a == 2 {
        println!("  ");
        println!("Пока находится в разработке! Приходите чуть-позже!");
        main();
    } else {println!("  "); println!("Введено неверное значение! Используйте цифры написанные выше!");}

    pause();
}