
pub fn run() {
    let val: String = String::from("Hello, World!");
    printer(&val);
    printer(&val);


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
    
    let mut name = "Karol".to_string();
    eprintln!("name before change = {:#?}", name);
    change_name(&mut name);
    eprintln!("changed name = {:#?}", name);
    let mut name2 = "Zenon".to_string();
    name2 = change_name2(name2);
    eprintln!("name2 = {:#?}", name2);
}

fn change_name(name: &mut String){
    *name = String::from("New name");
}

fn change_name2(mut name: String) -> String {
    name = String::from("New name change name2");
    name
}

fn printer(val: &String) {
    println!("The value is: {}", val);
}