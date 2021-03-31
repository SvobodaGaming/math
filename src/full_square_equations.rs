use read_input::prelude::*;

pub fn full_square_equations() {
    println!("  ");
    println!("  ");
    println!("  ");
    println!("Отличный выбор!");
    println!("  ");
    let a = input::<f64>().msg("Введите значение переменной A: ").err("Введено неверное значение! Повторите попытку!").get();
    let b = input::<f64>().msg("Введите значение переменной B: ").err("Введено неверное значение! Повторите попытку!").get();
    let c = input::<f64>().msg("Введите значение переменной C: ").err("Введено неверное значение! Повторите попытку!").get();
    println!("  ");
    println!("a = {}; b = {}, c = {}",a,b,c);
    println!("  ");
    let d: f64 = b*b;
    let e: f64 = 4.0*a*c;
    let sign: char;
    if e > 0.0 {
        sign = '-';
    } else if e < 0.0 {
        sign = '+';
    } else { sign = 'd';}
    let di: f64 = d-(4.0*a*c);
    let dl: f64 = di.sqrt();
    let x: char;
    let bl: f64 = -b;
    println!("D = b^2 - 4 * a * c; D = ({}^2) - 4 * {} * {} = ({}) {} {} = {};",b,a,c,d,sign,e,di);
    if di > 0.0 {
        println!("D > 0 => корня 2");
        x = 'T';
    } else if di == 0.0 {
        println!("D = 0 => корень 1");
        x = 'O';
    } else {
        println!("D < 0 => корней нет");
        x = 'N';
    }
    println!("  ");
    println!("         -b + - √D ");
    println!("x1,2 = ------------ ;");
    println!("           2 * a       ");
    println!("  ");
    println!("       {} + √{}         {} + {}             {}",bl,di,bl,dl,bl+dl);
    println!("x1 = ------------- = -------------- = --------------- =   {} ;", (bl + dl)/(2.0*a));
    println!("         2 * a            2 * {}             {}",a,2.0*a);
    println!("  ");
    if di != 0.0 {
        println!("       {} - √{}          {} - {}            {}",bl,di,bl,dl,bl-dl);
        println!("x2 = ------------- = -------------- = ------------- =   {} ;", (bl - dl)/(2.0*a));
        println!("         2 * a            2 * {}            {}",a,2.0*a);
    }
    println!("  ");
    if x == 'T' {
        println!("Ответ: {} ; {}",(bl + dl)/(2.0*a),(bl - dl)/(2.0*a));
    } else if x == 'O' {
        println!("Ответ: {}",(bl + dl)/(2.0*a));
    } else if x == 'N' {
        println!("Корней нет!");
    } else {
        println!("Возникла ошибка при решении уравнения! Повторите попытку.");
    }
    println!("  ");
}