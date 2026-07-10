use std::io;

fn main() {
    println!("do you like dogs? [Y/n]");

    let mut index = String::new(); // create an empty string
    let _ = io::stdin().read_line(&mut index); // read for STDIN
    let index = index.trim().chars().next().unwrap_or('y'); //save the first char of the string

    match index {
        //match the char
        'y' => println!("yay!"),
        'n' => println!("nay :("),
        'Y' => println!("yay!!"),
        'N' => println!("nay :("),
        _ => println!("yay!"),
    }

    println!("{index}")
}
