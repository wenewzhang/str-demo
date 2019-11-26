fn main() {
    let hello = "123";

    let s = &hello[0..1];
    //let s = &hello[0..1]; error
    println!("{}",s);

    for b in "我爱你，儿子们！".chars() {
        println!("{}", b);
    }
    for b in "我爱你，儿子们！".bytes() {
        println!("{}", b);
    }
}
