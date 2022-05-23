use std::process::exit;
use rand;
use rand::seq::SliceRandom;

fn main() {
    println!("Быки и коровы!");

    let mut gen = rand::thread_rng();
    //let secret: Vec<i32>  = (0..4).map(|_| {gen.gen_range(1..10)}).collect();
    let secret: [i32; 4] = {
        let mut to_gen = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut secret: [i32; 4] = [0; 4];
        for i in 0..secret.len() { // Генерация 4 не повторяющихся случайных цифр, со сложностью O(n).
            secret[i] = *to_gen.choose(&mut gen).unwrap();
            let index = to_gen.iter().position(|x| *x == secret[i]).unwrap(); // Не до конца понял (спиздил со stackoverflow)
            to_gen.remove(index);
        }
        secret
    }; // Сверху за комментирована генерация в вектор. НО Я ХОЧУ В МАССИВ. Более короткого способа я не знаю.

    //println!("{:?} ", secret);
    //Раскомментируйте выше, чтобы просмотреть сгенерированную задачу.
    let mut tries = 0;
    println!("Угадайте 4 загаданных числа [1..9], у вас 10 попыток");
    loop {
        println!("Введите 4 цифры через пробел: ");
        let r = get_num();
        //println!("{:?}", r);
        let mut cows = 0;
        let mut bools = 0;
        for i in 0..r.len() {
            for j in 0..secret.len() {
                if r[i] == secret[j] {
                    if i == j {
                        bools += 1;
                    } else {
                        cows += 1;
                    }
                }
            }
        }
        println!("{} быка и {} коровы", bools, cows);
        if bools == 4 {
            println!("Вы выиграли");
            exit(0);
        }
        if tries > 9 {
            println!("Вы проиграли, было загадано {:?} ", secret);
            exit(0);
        }
        tries += 1;
    }
}

fn get_num() -> [i32; 4] { // Читает вектор в массив, по факту можно и без неё, НО Я ХОЧУ В МАССИВ.
    let x = read_vec("Некорректный ввод, попробуйте ещё раз: ");
    let mut res: [i32; 4] = [0; 4];
    for i in 0..4 {
        res[i] = x[i];
    }
    res
}

fn read_vec(on_err: &str) -> Vec<i32> { // Считывает 4 числа в вектор. Отдельная функция только чтобы потом мб юзать в других проектах
    'outer: loop {
        let inp = read_str();
        let mut res = vec![];
        for e in inp.trim().split(" ") {
            if e.len() > 1 {
                println!("{}", on_err);
                continue 'outer;
            }
            if !e.parse::<i32>().is_err() {
                res.push(e.parse::<i32>().unwrap())
            } else {
                println!("{}", on_err);
                continue 'outer;
            }
        }
        if res.len() != 4 {
            println!("{}", on_err);
            continue 'outer;
        }
        return res;
    }
}


fn read_str() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).expect("Произошла непредвиденная ошибка");
    return buf;
}


// fn read_integer() -> i32 {
//     loop {
//         let buf = read_str();
//         match buf.trim().parse()
//         {
//             Ok(num) => return num,
//             Err(_) => continue,
//         };
//     }
// }
