# FizzBuzz

## What is FizzBuzz (In programming)
Fizz buzz (often spelled FizzBuzz in this context) has been used as an interview screening device for computer programmers. Writing a program to output the first 100 FizzBuzz numbers is a relatively trivial problem requiring little more than a loop and conditional statements. However, its value in coding interviews is to analyze fundamental coding habits that may be indicative of overall coding ingenuity.
</br>
[Wikipedia](https://en.wikipedia.org/wiki/Fizz_buzz)

## My solution
Tipicaly FizzBuzz is solved using modulo operator ("%") for all the filtering (3, 5, 15) as show in this example written in Java.
```java
public class FizzBuzz {
    public static void main(String[] args) {
        int i;
        for (i = 1; i <= 100; i++) {
            if (i % 15 == 0)
                System.out.print("fizzbuzz ");
            else if (i % 3 == 0)
                System.out.print("fizz ");
            else if (i % 5 == 0)
                System.out.print("buzz ");
            else
                System.out.print(i + " ");
        }
    }
}
```

In my case, since I wanted to use the string concatenation in Rust, with the method "push_str()", to learn about it, I came up with this:
```rust
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
```