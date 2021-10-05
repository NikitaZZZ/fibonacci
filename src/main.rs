use std::io;

fn main() {
    let mut result_array: Vec<usize> = Vec::new();

    println!("Enter the index of the number to calculate the fibonacci .");
    let mut fibonacci_number = String::new();
    io::stdin()
        .read_line(&mut fibonacci_number)
        .expect("Error reading input");

    let fibonacci_number: usize = fibonacci_number.trim().parse().expect("Error parsing number");

    for i in 0..fibonacci_number+1 {
        if i == 0 || i == 1 { result_array.push(i); }
        else {
            let result = result_array[i-1] + result_array[i-2];
            result_array.push(result);
        }
    }

    println!("Result is - {}", result_array[result_array.len() - 1]);
}
