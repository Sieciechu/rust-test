use std::io;
use std::io::Read;
use std::fs::File;

pub fn run(){
    let text = read_username_from_file("hello.txt").unwrap();
    eprintln!("read text from file:\n\"{}\"", text);

    let text = read_username_from_file("not_existing_file").unwrap(); // this causes panic

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