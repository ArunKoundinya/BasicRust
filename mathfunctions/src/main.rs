fn main() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5]; // Create a vector of integers

    // Pass a reference to the vector so that `numbers` can be reused
    let sum_results = mathfunctions::add_numbers(&numbers);
    let prod_results = mathfunctions::multiple_numbers(&numbers);
    let divide_results = mathfunctions::our_division(&numbers);

    println!("The sum is: {}", sum_results);
    println!("The product is: {}", prod_results);
    match divide_results {
        Some(result) => println!("The result of the division is: {}", result),
        None => println!("Cannot divide by zero."),
    }
}
