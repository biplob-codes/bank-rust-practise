fn print_elements(elements: &Vec<String>) {
    for e in elements {
        println!("{e}")
    }
}
fn main() {
    let fruits = vec![
        String::from("apple"),
        String::from("orange"),
        String::from("pineapple"),
        String::from("grape"),
    ];
    print_elements(&fruits);
}
