use std::collections::HashMap;
use std::{array, hash, string};

#[allow(dead_code, unused_imports)]

pub fn string_to_number(s: &str) -> i32 {
    let my_int = s.parse::<i32>().unwrap();
    return my_int;
}

pub fn digitize(n: u64) -> Vec<u8> {
    let from_number_string = n.to_string();
    let mut forward_list: Vec<u8> = vec![];
    print!("{}", from_number_string);
    for c in from_number_string.chars().rev() {
        let digit = String::from(c);
        let ch: u8 = digit.parse::<u8>().unwrap();
        forward_list.push(ch);
    }
    forward_list
}

// USing digitize but with best practices
fn digitize_1(n: u64) -> Vec<u8> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .rev()
        .collect::<Vec<u8>>()
}

pub fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    let mut count_positive_num: i16 = 0;
    let mut sum_negatives: i32 = 0;
    let mut res: Vec<i32> = vec![];
    if input.len() == 0 {
        return input;
    }

    for (i, &x) in input.iter().enumerate() {
        if x > 0 {
            count_positive_num += 1;
        } else {
            sum_negatives += x;
        }
    }
    res.push(count_positive_num.into());
    res.push(sum_negatives.into());
    res
}

// // Using digitize but with best practices
fn count_positives_sum_negatives_1(input: Vec<i32>) -> Vec<i32> {
    if input.is_empty() {
        return vec![];
    }

    input.iter().fold(vec![0, 0], |mut acc, &x| {
        if x > 0 {
            acc[0] += 1;
        } else {
            acc[1] += x;
        }
        acc
    })
}

// pub fn square_digits(num: u64) -> u64 {
//     let square_digits =
//         num
//         .to_string()
//         .chars()
//         .map(|c| c.to_digit(10).unwrap() as u8)
//         .rev()
//         .collect::<Vec<u8>>()
// }

pub fn square_digits1(num: u64) -> u64 {
    let digits = num
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect::<Vec<u64>>();

    let squares = digits.into_iter().map(|c| c * c);
    let squares_up: Vec<String> = squares.map(|c| c.to_string()).collect();
    let joined_res = squares_up.join("");
    joined_res.parse::<u64>().unwrap()
}

// Better solutions for squared digits
fn square_digits(num: u64) -> u64 {
    num.to_string()
        .chars()
        .map(|i| i.to_digit(10).expect("char isnt digit").pow(2).to_string())
        .collect::<String>()
        .parse()
        .expect("result isnt u64 parsable")
}

pub fn reverse_words(str: &str) -> String {
    // your code here
    let splited: Vec<&str> = str.split(" ").collect();
    let converted_texts: Vec<String> = splited
        .into_iter()
        .map(|c| c.chars().rev().collect::<String>())
        .collect();
    converted_texts.join(" ")
}

pub fn likes(names: &[&str]) -> String {
    let results: String = match names.len() {
        0 => String::from("no one likes this"),
        1 => {
            let name_1 = names[0].to_string();
            name_1 + &" likes this".to_string()
        }
        2 => {
            let name_1 = names[0].to_string();
            let name_2 = names[1].to_string();
            name_1 + &" and ".to_string() + &name_2 + &" like this".to_string()
        }
        3 => {
            let name_1 = names[0].to_string();
            let name_2 = names[1].to_string();
            let name_3 = names[2].to_string();
            name_1
                + &", ".to_string()
                + &name_2
                + &" and ".to_string()
                + &name_3
                + &" like this".to_string()
        }
        _ => {
            let names_length = (names.len() - 2).to_string();
            let name_1 = names[0].to_string();
            let name_2 = names[1].to_string();
            name_1
                + &", ".to_string()
                + &name_2
                + &" and ".to_string()
                + &names_length
                + &" others like this".to_string()
        }
    };
    return results;
}

// Updated likes with best practices
fn likes_1(names: &[&str]) -> String {
    match names {
        [] => format!("no one likes this"),
        [a] => format!("{} likes this", a),
        [a, b] => format!("{} and {} like this", a, b),
        [a, b, c] => format!("{}, {} and {} like this", a, b, c),
        [a, b, rest @ ..] => format!("{}, {} and {} others like this", a, b, rest.len()),
    }
}

pub fn alphabet_position(text: &str) -> String {
    let alphabet: String = "abcdefghijklmnopqrstuvwxyz".to_string();
    let text_updated = text.to_lowercase();
    let mut results_indexe = String::new();
    for e in text_updated.chars() {
        if alphabet.contains(e) {
            let i = alphabet.find(e).unwrap_or(0);
            results_indexe = results_indexe + &(i + 1).to_string() + " ";
            // println!("{}", i);
        }
    }
    return results_indexe;
}

// fn alphabet_position(text: &str) -> String {
//     text.to_lowercase()
//         .chars()
//         .filter(|c| c >= &'a' && c <= &'z')
//         .map(|c| (c as u32 - 96).to_string())
//         .collect::<Vec<String>>()
//         .join(" ")
// }

pub fn count_duplicates(text: &str) -> u32 {
    // Your code goes here
    let mut count: u16 = 0;
    let mut alones: Vec<String> = Vec::new();
    let text_x = text.to_lowercase();
    for (indx, e) in text_x.chars().enumerate() {
        // let subs: String = text.chars().skip(index).take(text.len() - index).collect();
        let subs: &str = &text_x[indx + 1..text_x.len()];
        if !alones.contains(&e.to_string()) && subs.contains(e) {
            count = count + 1;
        }
        alones.push(e.to_string());
    }
    count as u32
}

pub fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
    let mut arr: Vec<String> = Vec::new();
    for e in arr_a.iter() {
        for tr in arr_b.iter() {
            if tr.len() >= e.len() && tr.contains(e) && !arr.contains(&e.to_string()) {
                arr.push(e.to_string());
                break;
            }
        }
    }
    arr.sort_by(|a, b| a.partial_cmp(b).unwrap());
    arr
}

// fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
//     let mut result: Vec<String> = arr_a.iter()
//         .filter(|&e| arr_b.iter().any(|&t| t.contains(e)))
//         .map(|s| s.to_string())
//         .collect();

//     result.sort_unstable();
//     result.dedup();
//     result
// }

pub fn dig_pow(n: i64, p: i32) -> i64 {
    // let binding = n.to_string();
    let mut p_int = p - 1;
    let mut sums: i64 = n
        .to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .map(|c| {
            p_int = p_int + 1;
            c.pow(p_int as u32)
        })
        // .sum::<i64>();
        .sum::<i64>();

    match (sums % n == 0) {
        true => return sums / n as i64,
        false => -1,
    }
}

pub fn count(input: &str) -> HashMap<char, i32> {
    let mut letter_counts = HashMap::new();
    let char_vec: Vec<char> = input.chars().collect();
    for e in char_vec {
        *letter_counts.entry(e).or_insert(0) += 1;
    }
    return letter_counts;
}
