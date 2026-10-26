// Simple program for Arithmetic Progression

use std::io;

// Calculates the difference between two terms
fn difference(t1 : i32, t2 : i32) {
    let diff : f64 = (t2 - t1).into();
    println!("\nThe difference is : {}\n", diff);
}

// Calculate sum of n terms using last term
fn sum_of_ap(a : i32, d : i32, n : i32) {
    let sum : i32 = n/2*((2*a) + ((n-1)*d));
    println!("\nSum of AP is : {}\n", sum);
}

// Calculate sum of n terms without using last term
fn sum_of_ap_2(a : i32, l : i32, n : i32) {
    let sum : i32 = n/2*(a + l);
    println!("\nSum of AP is : {}\n", sum);
}

// Calculate nth term of AP
fn nth_term(a : i32, d : i32, n : i32) {
    let term : i32 = a + ((n-1)*d);
    println!("\nThe nth term of AP is : {}\n", term);
}

// Prints the menu
fn menu() {
    let string = "\nSelect appropriate number to perform an operation :
1 : Find the difference
2 : Find the nth term
3 : Sum of AP (using last term)
4 : Sum of AP (without using last term)
5 : Exit\n";

println!("{}", string);
}

// Main code block
fn main() {
    menu();

    let mut input = String::new(); // variable for taking inputs

    loop { // Infinite loop for taking inputs and performing operations

        // Takes the input for command from user
        io::stdin().read_line(&mut input).unwrap();
        let command : i32 = input.trim().parse().unwrap();

        if command == 1 {
            println!("\nEnter any term of the AP:");
            input.clear();
        
            io::stdin().read_line(&mut input).unwrap();
            let term1 : i32 = input.trim().parse().unwrap();

            println!("\nNow Enter next term of AP:");
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let term2 : i32 = input.trim().parse().unwrap();

            difference(term1, term2);

        } else if command == 2 {
            println!("\nEnter first term of AP :");
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let a : i32 = input.trim().parse().unwrap();

            println!("\nEnter the difference:");
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let d : i32 = input.trim().parse().unwrap();

            println!("\nEnter the value of n:");
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let n : i32 = input.trim().parse().unwrap();

            nth_term(a, d, n);

        } else if command == 3 {
            println!("\nEnter first term of AP :");
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let a : i32 = input.trim().parse().unwrap();

            println!("\nEnter the value of n");
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let n : i32 = input.trim().parse().unwrap();

            println!("\nEnter the last term of AP ");
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let l : i32 = input.trim().parse().unwrap();

            sum_of_ap_2(a, l, n);

        } else if command == 4 {
            println!("\nEnter first term of AP :");
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let a : i32 = input.trim().parse().unwrap();

            println!("\nEnter the value of n");
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let n : i32 = input.trim().parse().unwrap();

            println!("\nEnter the difference");
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            let d : i32 = input.trim().parse().unwrap();

            sum_of_ap(a, d, n);

        } else if command == 5 {
            println!("\nExiting the program....");
            break

        } else {
            println!("\nNot an appropriate command");
            menu();
        }

        input.clear();
    }
}
 
// TODO : add 'find first term' in future