fn main() {
    let fruits: Vec<&str> = vec!["Apple", "Banana", "Mango", "Orange"];
    for fruit in fruits.iter().rev() {
        println!("{fruit}");
    }
}
