use rand::Rng; //traits
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number !");

    /*
      - upper bound not included: it will produce
        the number from 1 to 100
    */
    let secret_number = rand::thread_rng().gen_range(1..101);

    /* infinite loop */
    loop {
        /*
          - "guess" will receive the user input hence needs
            to be mutable
        */
        let mut guess = String::new();

        /*
          - reference by default is immutable
            so you need to make it mutable
            so the "guess" can be changed
          - read_line will return a Result type
            - if everything is fine, nothing
            would happen by calling the expect
            method
            - if there is any error, calling
            expect will crash the program
        */
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        /*
          - use match expression to handle Result
            type
          - we use shadowing here so now the type of
            "guess" is now u32 (it was a String before)
          - parse needs your help in type inference
            so it knows what you are parsing into;
            in this case we annote "guess" as a u32
            to help it out
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        /*
          - use match to handle Enum type
        */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
