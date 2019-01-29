extern crate rand;

use add_one::{
    add_one
};

const RAND_MIN:i32 = 0;
const RAND_MAX:i32 = 20;


fn add_random(x: i32) -> i32 {
    use rand::{thread_rng, Rng};
    x + thread_rng().gen_range(RAND_MIN, RAND_MAX)
}


fn main() {
    let x = 16;

    println!("{:?} + 1 = {:?}", x, add_one(x));
    println!("{:?} + random = {:?}", x, add_random(x));
}
