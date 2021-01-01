fn main() {
    let s = String::from("hello");
    takes_ownewship(s);

    let x = 5;
    makes_copy(x);

    println!("{}",s) //this syntax is illegal beacause the function takes_ownewship take the ownership of the string s

}

fn takes_ownewship(s: String){
    println!("{}",s);
}
fn makes_copy(i:i32){
    println!("{}",i);
}