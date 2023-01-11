/////////////////////////
//  Генератор паролей  //
// Vladimir N Platonov //
//    Moscow, 2022     //
/////////////////////////
// echo "$(passgen 15 1)" |awk '{print $2}'

use std::env;
use rand::Rng;
use base64::encode;
use std::process;

fn main() {
    let help_string="Программа - генератор паролей:\n  passgen [Длина пароля (8..256)|-] [Количество паролей (1..10000)|-] [Используемые наборы символов (1-4)|-]";
    
    //Длина пароля, Количество паролей, Используемые наборы символов
    let mut pass_parrams: Vec<u32> = vec![ 9, 10, 4];

    // Признак вывода помощи
    let mut help_check=false;

    // Счт=итываем аргументы командной строки
    let args: Vec<String> = env::args().collect();

    for i in 1..4 {
        if let Some(el) = args.get(i) {
            match el.trim().parse::<u32>() {
                Ok(el) => {
                    if i-1 == 0 {
                        match el {
                            ..=7 => continue,
                            8..=256 => pass_parrams[i - 1] = el,
                            _ => pass_parrams[i - 1] = el % 256,
                        }
                    } else if i-1 == 1 {
                        match el {
                            0..=10000 => pass_parrams[i - 1] = el,
                            _ => continue,
                        }
                    } else if i-1 == 2 {
                        if el > 1 && el <= pass_parrams[2] {
                            pass_parrams[i - 1] = el
                        }
                    }
                }
                Err(_) => {
                    if el=="-h" || el=="--help" {
                        help_check=true
                    } else {
                        continue
                    }
                }
        }
        } else {
            // println!("Ошибка парсинга аргументов командной строки!")
        }
    }

    if help_check==true {
        println!("{}", help_string);
        process::exit(0);
    }

    // Цифры
    let str_num: Vec<char> = vector_gen('0', '9');

    // Большие латинские буквы
    let str_lat_b:Vec<char> = vector_gen('A', 'Z');

    // Маленькие латинские буквы
    let str_lat_s:Vec<char> = vector_gen('a', 'z');

    // Спецсимволы
    let str_spec: Vec<char> = vec!['+', '-', '&', '*', '^', '%', '$', '#', '@', '!', '~', '`', '.', ',', '/'];


    for _j in 0..pass_parrams[1] {     // Цикл по количеству паролей
        
        let mut str_pass: Vec<char>  = vec![];
        let mut ch_index: u32;
        
        for k in 0..pass_parrams[0] {          // Цикл по длине пароля - 1 
            if k != 0 {
                ch_index = rand::thread_rng().gen_range(1..=pass_parrams[2]);
            } else {
                ch_index = rand::thread_rng().gen_range(1..=2);
            }
            match ch_index {
                1 => {
                    str_pass.push(char_gen(&str_lat_b))
                },
                2 => {
                    str_pass.push(char_gen(&str_lat_s))
                },
                3 => {
                    str_pass.push(char_gen(&str_num))
                },
                4 => {
                    str_pass.push(char_gen(&str_spec))
                },
                _ => {
                    println!("Задан недопустимый набор символов!");
                    break;
                },
            }

        }

        let password = &str_pass.iter().collect::<String>();
        let encoded = encode(password);
        println!("{password}    {encoded}");
    }

}

// Генератор набора символов
fn vector_gen(char_start: char, char_end: char) -> Vec<char> {
    let mut vec_out: Vec<char> = vec![];
    for ch in char_start..=char_end { 
        vec_out.push(ch);
    }
    return vec_out;
}

// Генератор случайного символа
fn char_gen(char_vec: &Vec<char>) -> char {
    let len_vec = char_vec.len();
    let rnd_ch_index = rand::thread_rng().gen_range(0..len_vec);
    char::from(char_vec[rnd_ch_index])
}