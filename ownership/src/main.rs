fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

    let s3 = String::from("my string");
    let len = calculate_length_ref(&s3);

    println!("The length of '{}' is {}.", s3, len);

    // String slices
    let my = &s3[0..2];
    let st = &s3[3..9];

    println!("my: {} and st: {}", my, st);
}

// owner ship transfer to function and then return back to caller
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

// pass ownership as reference so no need to return the string back
fn calculate_length_ref(s: &String) -> usize {
    s.len()
}
