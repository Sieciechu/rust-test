
#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


fn main() {
    let mut v1: Vec<i32> = Vec::new();

    v1.push(12);

    let v2 = vec![1,2,3];

    let mut ar = [v1, v2];

    ar[0].push(12);


    let v3 = vec![10,20,30,40];
    let mut third: &i32 = &v3[2];

    let x = v3[1];
    eprintln!("x = {:#?}", x);

    third = &23;


    let index = 20;

    match v3.get(index) {
        Some(_) => println!("Reachable element at index {}", index),
        None => println!("Unreachable element at index {}", index)
    }

    eprintln!("v3 = {:?}", v3);
    eprintln!("third = {:#?}", third);


    for i in &v3 {
        eprintln!("i = {:#?}", i);
    }

    let mut v4 = vec![100, 32, 57];
    for i in &mut v4 {
        *i += 50;
    }
    eprintln!("v4 after loop is = {:#?}", v4);

    let mut row = vec![
        SpreadsheetCell::Int(200),
        SpreadsheetCell::Float(300.12),
        SpreadsheetCell::Text("blue".to_string()),
    ];
    
    let cell = SpreadsheetCell::Int(201);
    row.push(cell);

    eprintln!("row is: {:?}", row);
}
