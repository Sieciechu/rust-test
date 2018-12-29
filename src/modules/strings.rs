
pub fn run() {
    let s1 = String::from("john");
    let s2 = String::from("smith");
    let s3 = "tall".to_string();
    let s4 = format!("I am {} {}, and I'm {}", s1, s2, s3);
    eprintln!("s1 = {:#?}", s1);
    eprintln!("s2 = {:#?}", s2);
    eprintln!("s3 = {:#?}", s3);
    println!("s4 = {}", s4);

    let s5 = String::from("Здравствуйте");
    eprintln!("size of Здравствуйте is = {:#?}", s5.len());
    eprintln!("first two chars of Здравствуйте are \"{}\"", &s5[0..4]);

    for c in "नमस्ते".chars() {
        eprintln!("c = {:#?}", c);
    }

    for (index, byte) in "नमस्ते".bytes().enumerate() {
        eprintln!("byte[{}] = {:#?}", index, byte);
    }
}
