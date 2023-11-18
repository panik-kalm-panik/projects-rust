fn main() {
    for n in 1..101 {
        let mut output_string: String = "".to_string();
        if n % 3 == 0 {
            output_string.push_str("fizz");
        }
        if n % 5 == 0{
            output_string.push_str("buzz");
        }
        if output_string == "" {
            output_string = n.to_string();
        }
        println!("{}", output_string);
    }
}