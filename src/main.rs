
fn main() {

    let thread1 = std::thread::spawn(|| {
        loop{
            print!("thread111");
            std::thread::sleep(std::time::Duration::from_secs(1));
            
        }

    });

    let thread2 = std::thread::spawn(|| {
        loop {
            print!("thread22222");
            std::thread::sleep(std::time::Duration::from_secs(1));
            
        }
    });

    let thread3 = std::thread::spawn(|| {
        loop {
            println!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>");
            std::thread::sleep(std::time::Duration::from_secs(1));
            
        }
    });

    thread1.join();
    thread2.join();
    thread3.join();


}
