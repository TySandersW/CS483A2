use std::io;
use rand::Rng;

fn main() {
    loop{
        println!("Please input your text:");

        let mut text = String::new();

        io::stdin()
            .read_line(&mut text)
            .expect("Failed to read line");

        let text = text
            .trim().to_lowercase().chars()
            .filter(|c| c.is_alphanumeric()).collect();

        if text == "q" {
            return;
        }

        let new_text = input_func(text);

        println!("The new text is: {}", new_text);
    }
}

fn input_func(text: String) -> String {
    println!("Press 'd' to delete a random letter");
    println!("Press 'r' to replace a random letter");
    println!("Press 'i' to insert a random letter");
    println!("press 'q' to quit");

    let mut func = String::new();

    io::stdin()
        .read_line(&mut func)
        .expect("Failed to read line");

    let func: String = func
        .trim().to_lowercase().chars()
        .filter(|c| c.is_alphanumeric()).collect();

    if func == "d" {
        del_let(text)
    }
    else if func == "r" {
        rep_let(text)
    }
    else if func == "i" {
        ins_let(text)
    }
    else {
        text
    }
}

fn del_let(mut text: String) -> String {
    let mut rng = rand::thread_rng();

    for _ in 0..5 {
        if text.is_empty() {
            break;
        }

        let letter = rng.gen_range(0..text.len());
        text.remove(letter);
    }
    text

}

fn rep_let(mut text: String) -> String {
    let mut rng = rand::thread_rng();

    for _ in 0..5 {
        if text.is_empty() {
            break;
        }

        let letter = rng.gen_range(0..text.len());
        text.remove(letter);

        let rng_char = rng.gen_range(b'a'..=b'z') as char;
        text.insert(letter, rng_char);
    }
    text
}

fn ins_let(mut text: String) -> String {
    let mut rng = rand::thread_rng();

    for _ in 0..5 {
        if text.is_empty() {
            break;
        }

        let letter = rng.gen_range(0..=text.len());
        let rng_char = rng.gen_range(b'a'..b'z') as char;
        text.insert(letter, rng_char);
    }
    text
}