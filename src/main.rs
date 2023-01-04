/////////////////////////
//  Генератор паролей  //
// Vladimir N Platonov //
//    Moscow, 2022     //
/////////////////////////

use std::env;
use rand::Rng;
use base64::encode;

fn main() {
    let mut password_length =  9; // Длина пароля
    let mut password_count  = 10; // Количество паролей
    let mut password_chars  =  4; // Используемые наборы символов

    let args: Vec<String> = env::args().collect();

    match args.len() {
        2 => {
            let password_length_str = &args[1];
            password_length = password_length_str.parse().unwrap();
        },
        3 => {
            let password_length_str = &args[1];
            password_length = password_length_str.parse().unwrap();
            let password_count_str = &args[2];
            password_count = password_count_str.parse().unwrap();
        },
        4 => {
            let password_length_str = &args[1];
            password_length = password_length_str.parse().unwrap();
            let password_count_str = &args[2];
            password_count = password_count_str.parse().unwrap();
            let password_chars_str = &args[3];
            password_chars = password_chars_str.parse().unwrap();
        },
        _ => {}
    }

    // Цифры
    let str_num: Vec<char> = vector_gen('0', '9');
    let len_num = str_num.len();

    // Большие латинские буквы
    let str_lat_b:Vec<char> = vector_gen('A', 'Z');
    let len_b = str_lat_b.len();

    // Маленькие латинские буквы
    let str_lat_s:Vec<char> = vector_gen('a', 'z');
    let len_s = str_lat_s.len();

    // Спецсимволы
    let str_spec: Vec<char> = vec!['+', '-', '&', '*', '^', '%', '$', '#', '@', '!', '~', '`', '.', ',', '/'];
    let len_spec = str_spec.len();


    for _j in 0..password_count {     // Цикл по количеству паролей
        
        let mut str_pass: Vec<char>  = vec![];
        
        // Первый символ - буква
        let mut ch_index = rand::thread_rng().gen_range(1..=2);
        match ch_index {
            1 => {
                str_pass.push(char_gen(len_b, &str_lat_b));
            },
            2 => {
                str_pass.push(char_gen(len_s, &str_lat_s));
            },
            _ => {
                println!("Задан недопустимый набор символов!");
                break;
            },
        }

        for _ in 1..password_length {
            ch_index = rand::thread_rng().gen_range(1..=4);
            match ch_index {
                1 => {
                    str_pass.push(char_gen(len_b, &str_lat_b));
                },
                2 => {
                    str_pass.push(char_gen(len_s, &str_lat_s));
                },
                3 => {
                    let rnd_ch_index = rand::thread_rng().gen_range(0..len_num);
                    str_pass.push(char::from(str_num[rnd_ch_index]))
                },
                4 => {
                    let rnd_ch_index = rand::thread_rng().gen_range(0..len_spec);
                    str_pass.push(char::from(str_spec[rnd_ch_index]))
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

fn vector_gen(char_start: char, char_end: char) -> Vec<char> {
    let mut vec_out: Vec<char> = vec![];
    for ch in char_start..=char_end { 
        vec_out.push(ch);
    }
    return vec_out;
}

fn char_gen(len_vec: usize, char_vec: &Vec<char>) -> char {
    let rnd_ch_index = rand::thread_rng().gen_range(0..len_vec);
    char::from(char_vec[rnd_ch_index])
}