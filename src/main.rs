fn main() {
    for i in 0..101 {
        if (i % 3 == 0) {
            print!("Fizz");
        }
        if (i % 5 == 0) {
            print!("Buzz");
        }
        else if (i % 3 !=0 && i % 5 != 0) {
            print!("{}", i.to_string());
        }
        println!();
    }
}
