use std;

pub fn run(){
    use std::collections::HashMap;

    let teams = vec![
        String::from("a team"),
        String::from("b team"),
    ];

    let points = vec![20, 30];

    let scores: HashMap<_, _> = teams.iter().zip(points.iter()).collect();
    eprintln!("scores = {:#?}", scores);


    let name = "john".to_string();
    let age: u32 = 32;
    let mut men = HashMap::new();
    men.insert(name, age);
    // eprintln!("name = {:#?}", name); // can't use because name is moved
    eprintln!("year = {:#?}", age);
    men.insert("adam".to_string(), 21);
    men.insert("carol".to_string(), 44);

    print_men(&men);
    eprintln!("men = {:#?}", men);

    for (key, value) in &men {
        eprintln!("for loop, man named {} is of age {}", key, value);
    }


    let not_existing_man = "Johnnn".to_string();
    eprintln!("not_existing_man = {:#?}", not_existing_man);

    let man = men.get(&not_existing_man);

    eprintln!("{} is {:#?}", not_existing_man, man.unwrap_or(&12));
    eprintln!("{} is {:#?}", not_existing_man, man.unwrap_or_else(|| &20));
    let message = match man {
        Some(i) => i.to_string(),
        None => String::from("there no such guy like johnnnn"),
    };
    eprintln!("with assigned matched variable = {:#?}", message);

    eprintln!("short form = {:#?}", match men.get(&not_existing_man) {
        Some(i) => i.to_string(),
        None => format!("there no such guy like {}", &not_existing_man),
    });

    count_words_in_string(&format!("Hello Mr! Are you ok? How are you Mr {}?", not_existing_man));
}

fn print_men(men: &std::collections::HashMap<String, u32>){
    for (i, (name, age)) in men.iter().enumerate() {
        eprintln!("man[{}] is named {} and his age is {}", i, name, age );
    }
}

fn count_words_in_string(text: &str){
    let mut map = std::collections::HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word.to_lowercase()).or_insert(0);
        *count +=1;
    }

    eprintln!("counted words = {:#?}", map);
}