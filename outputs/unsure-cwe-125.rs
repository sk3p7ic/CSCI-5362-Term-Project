fn get_value_from_array(array: &[i32], len: usize, index: isize) -> i32 {
    let value;

    // check that the array index is less than the maximum length of the array
    if index < len as isize {
        // get the value at the specified index of the array
        value = array[index as usize];
    } else {
        // if array index is invalid then output error message
        // and return value indicating error
        println!("Value is: {}", array[index as usize]);
        value = -1;
    }

    value
}

fn main() {
    let arr = [0, 1, 2, 3];
    get_value_from_array(&arr, 4, -1);
}