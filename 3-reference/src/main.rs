fn my_print<T: std::fmt::Display>(msg: T) {
    println!("{}", msg);
}

fn main() {
    let m = "Hello".to_string();
    my_print(m.clone());
    my_print(m);
}
