fn main() {
    print_number(5);

    print_number(add_one(6));
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn add_one(x:i32) -> i32 {
    x + 1
}