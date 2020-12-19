use std::io;

fn main() {
    println!("please inputs the number.");
    let mut geuss = String::new();



    io::stdin()
        .read_line(&mut geuss)
        .expect("Failed to read line");
    println!("your input is: {}", geuss);
}
