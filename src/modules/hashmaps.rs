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

    for (key, value) in men {
        eprintln!("for loop, man named {} is of age {}", key, value);
    }
}

fn print_men(men: &std::collections::HashMap<String, u32>){
    for (i, (name, age)) in men.iter().enumerate() {
        eprintln!("man[{}] is named {} and his age is {}", i, name, age );
    }
}
