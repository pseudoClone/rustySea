fn main() {
    let s1 = String::from("Hello");
    let len = calculate_size(&s1);
    println!("The length of {} is {}.", s1, len);

    let mut s2 = String::from("Hello");
    change(&mut s2);
}

fn calculate_size(s: &String) -> usize {
    s.len()
}

fn change(s2: &mut String) {
    s2.push_str(", World");  // Only mutable references can be mutated
}

// cannot have two or mutable references to the same object
// cannot have a mutable reference while already immutable reference exists
