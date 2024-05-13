fn process_numbers(slice: &[i32]) {
    for (index, number) in slice.iter().enumerate() {
        println!("{}", number);
        if *number < 0 {
            panic!("Negative number found at index {}", index); // Stop execution and show error message
        }
    }
}

fn main() {
    let numbers = [1, 2, 3, -5];   // Include a negative number to trigger the panic
    process_numbers(&numbers);     // Call function with slice of integers as an argument
}