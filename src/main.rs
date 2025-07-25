use std::io::{stdin, stdout, Write};
use std::collections::linked_list;

const MAX_SEQUENCE: i64 = 1000;

fn main() {
    println!("| --- | Collatz Seq Generator | --- |");
    print!("Input a integer number: ");
    stdout().flush().unwrap();

    let mut input_str = String::new();
    stdin()
        .read_line(&mut input_str)
        .expect("Invalid Input");

    let input_num: i64 = match input_str.trim().parse() {
        Ok(value) => value,
        Err(_err) => {
            eprintln!("Input was invalid");
            return;
        },
    };

    // now we can generate the collatz sequence

    let mut collatz_seq: linked_list::LinkedList<i64> = linked_list::LinkedList::new();
    let mut highest_number = 0;
    let mut current_num = input_num;
    let mut iterations = 0;

    collatz_seq.push_back(current_num);

    while current_num != 1 {
        if is_even(current_num) {
            current_num = current_num / 2;
        } else {
            current_num = (3 * current_num) + 1;
        }

        collatz_seq.push_back(current_num);

        if current_num > highest_number {
            highest_number = current_num;
        }

        iterations += 1;
        if iterations >= MAX_SEQUENCE { // just incase it trys to run away to crazy amounts
            println!("C Sequence Exceed Max {} Iterations!", MAX_SEQUENCE);
            break;
        }
    }

    println!("Finished Calculating Collatz Sequence with a total of {} numbers.\nHighest Number: {}", collatz_seq.len()-1, highest_number); // minus one to account for the starting number that gets added
    
    print_collatz_link_list(collatz_seq);
    
    println!("Press Enter to close...");

    let mut wait = String::new();
    let _ = stdin().read_line(&mut wait);
}

fn print_collatz_link_list(list: linked_list::LinkedList<i64>) {
    let mut print_string = String::new();

    print_string = print_string + "Sequence:";

    for seq_num in list {
        print_string = print_string + " -> " + &seq_num.to_string();
    }

    println!("{}", print_string);
}

fn is_even(number: i64) -> bool {
    return (number % 2) == 0;
}