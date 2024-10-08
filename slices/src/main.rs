fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); //5 since func loop breaks at ' '

    s.clear();
    println!("{word}");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {  // Break at /x20
            return i;
        }
    }
    s.len()
}
