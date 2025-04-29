/**
 * Source: https://cwe.mitre.org/data/definitions/125.html
 */

fn get_value_from_array(array: &[i32], index: usize) -> i32 {
    let value;

    // check that the array index is less than the maximum length of the array
    if index < array.len() {
        // get the value at the specified index of the array
        value = array[index];
    } else {
        // if array index is invalid then output error message
        // and return value indicating error
        println!("Value is out of bounds");
        value = -1;
    }

    value
}

fn main() {
    let arr = [0, 1, 2, 3];
    let _ = get_value_from_array(&arr, 4); // using index 4 which is out of bounds
}