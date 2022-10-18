fn main() {
    let mut input = String::new();
    println!("Input string:");
    std::io::stdin()
        .read_line(&mut input)
        .expect("Input failed");
    let org_str = input.trim();

    let mut char_input = String::new();
    println!("Input character:");
    std::io::stdin()
        .read_line(&mut char_input)
        .expect("Input failed");
    let char_input = char_input.trim().to_string().pop().expect("Empty char");

    let mut res = String::new();

    for c in org_str.chars() {
        if char_input.to_ascii_lowercase() != c.to_ascii_lowercase() {
            res.push(c);
        }
    }

    println!("Result: {:?}", res);
}
