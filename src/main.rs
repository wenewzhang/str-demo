
fn main() {
    let result = calculate("MB","1.99");
    println!("{}", result)
}

fn calculate(unit:&str, num:&str) -> f64 {
    let mut cnum = 0.0;
    if unit == "MB" {
        cnum = num.parse::<f64>().unwrap();
        cnum *= 1024.0;
    } else if unit == "KB" {
        cnum = num.parse::<f64>().unwrap();
    }
    cnum
}
