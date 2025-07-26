fn main() {
    let s = String::from("Hello world!");
    let k = first_word(&s);
    println!("{k}");

    let m = "New word";
    let l = first_word(&m);
    println!("{l}");
}

fn first_word(s: &str) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    s
}
