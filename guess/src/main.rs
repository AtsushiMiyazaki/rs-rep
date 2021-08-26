use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {

    let rnum = rand::thread_rng().gen_range(0..101);

    loop {
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read line");

        let guess: u8 = buf.trim().parse();
        match guess.cmp(&rnum) {
            Ordering::Less => println!("Small"),
            Ordering::Equal => {
                println!("Exactly!");
                break;
            },
            Ordering::Greater => println!("Big"),
        }
    }
}
