fn main() {
    // Call println! with three arguments: a string, a value, a value
    let mut a_number;
    let a_word = "Ten";
    a_number = 10;
    println!("The number is {}.", a_number);
    println!("The word is {}.", a_word);
    a_number = 15;
    // Declare first variable binding with name "shadow_num"
    let shadow_num = 5;

    // Declare second variable binding, shadows existing variable "shadow_num"
    let shadow_num = shadow_num + 5;
    println!("The number is {}.", shadow_num);

    // Declare third variable binding, shadows second binding of variable "shadow_num"
    println!("The number is {}.", shadow_num);
    let shadow_num = shadow_num * 2;

    println!("The number is {}.", shadow_num);
}
