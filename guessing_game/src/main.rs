use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let  true_number = rand::thread_rng().gen_range(1,102);

    
    
    loop {
        
        let mut guess = String::new();

        println!("please input your guess");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 =match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&true_number) {
            Ordering::Equal => {
                println!("great");
                break;
            },
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
        }
    }
    println!("the true_number is {}",true_number);
}
 