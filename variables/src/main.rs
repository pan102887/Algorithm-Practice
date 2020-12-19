use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let mut x = 5;
    let mut buf = String::new();
    let tag = rand::thread_rng().gen_range(1,10);

    println!("the value of x is: {}", x);
    x = 6;
    println!("the value of x is: {}", x);

    loop {
        io::stdin()
            .read_line(&mut buf)
            .expect("failed to read line");
        
        let x:i32 = match buf.trim().parse(){
            Ok(x) => x,
            Err(_) => continue
        };

        match x.cmp(&tag) {
            Ordering::Equal => {
                println!("the tag is {}", tag);
                break;
            },
            Ordering::Less => {
                println!("the buf is {},and too small",buf);
            },
            Ordering::Greater => {
                println!("the buf is {},and too big",buf);
            }

        }
    }
}
