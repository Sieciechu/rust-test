use std::io;
use std::io::Read;
use std::fs::File;

pub fn run(){
    let text = read_username_from_file("hello.txt").unwrap();
    eprintln!("read text from file:\n\"{}\"", text);

    // let text = read_username_from_file("not_existing_file").unwrap(); // this causes panic
    let x = read_username_from_file3("hello.txt");
    eprintln!("x = {:#?}", x.unwrap());
}

fn read_username_from_file(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename);

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2(filename: &str) -> Result<String, io::Error> {
    let mut s= String::new();
    File::open(filename)?.read_to_string(&mut s)?;
    Result::Ok(s)
}

fn read_username_from_file3(filename: &str) -> Result<String, io::Error> {
    std::fs::read_to_string(filename)
}