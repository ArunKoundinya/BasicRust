// Accepting a slice instead of a reference to a Vec
pub fn add_numbers(mylist: &[i32]) -> i32 {
    mylist.iter().sum()
}

pub fn multiple_numbers(mylist: &[i32]) -> i32 {
    mylist.iter().product()
}

pub fn our_division(mylist: &[i32]) -> Option<i32> {
    let product: i32 = multiple_numbers(mylist);
    let sum: i32 =  add_numbers(mylist);
    if sum == 0 {
        return None; // Handle division by zero case
    }
    Some(product / sum)
}
