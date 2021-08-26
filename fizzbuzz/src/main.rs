fn main() {
    for i in 0..100 {
        let fizzbuzz = fizzbuzz(i);
        println!("number: {}\t{}", i, fizzbuzz)
    }
}

fn fizzbuzz(i: u16) -> String {
    if i % 15 == 0 {
        "fizzbuzz".to_string()
    } else if i % 3 == 0 {
        "fizz".to_string()
    } else if i % 5 == 0 {
        "buzz".to_string()
    } else {
        i.to_string()
    }
}
