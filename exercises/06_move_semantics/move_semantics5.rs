#![allow(clippy::ptr_arg)]

// TODO: Fix the compiler errors without changing anything except adding or
// removing references (the character `&`).

// Shouldn't take ownership
fn get_char(data: &str) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    // let str1 = "";
    let str1 = "Rust is great!";
    let data = str1.to_string();
    // let mut str2 = str1.to_string();

    // str2.push_str(" But C++ is good, too.");
    // println!("String slice: {str1}\nMutable String is {str2}");

    get_char(&data);

    string_uppercase(data);
}
