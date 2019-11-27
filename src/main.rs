fn print_me(msg: &str) { println!("msg = {}", msg); }

fn main() {
    let string = "hello world";
    print_me(string);

    let owned_string = "hello world".to_string(); // or String::from_str("hello world")
    print_me(&owned_string);

    let counted_string = std::rc::Rc::new("hello world".to_string());
    print_me(&counted_string);

    let atomically_counted_string = std::sync::Arc::new("hello world".to_string());
    print_me(&atomically_counted_string);
}
