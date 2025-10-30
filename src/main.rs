

fn main() {
    // 1. Basic Output
    println!("Hello, world!");
    println!("there is a exclamation mark in rust lang println.");

    // 2. Variables and Mutability
    let x = 5;
    println!("x = {}", x);

    // immutable by default:
    // x = 6; // will cause an error

    // use `mut` for mutable variables
    let mut y = 10;
    y += 5;
    println!("y = {}", y);

    // 3. Data Types
    let age: u32 = 21;
    let pi: f64 = 3.1415;
    let is_cool: bool = true;
    let initial: char = 'A';
    let name: &str = "Averi";

    println!("{} is {} years old.", name, age);
    println!("Pi: {}, Cool: {}, Initial: {}", pi, is_cool, initial);

    // 4. Constants
    const MAX_SCORE: i32 = 100;
    println!("Max score = {}", MAX_SCORE);

    // 5. Arrays & Vectors
    let arr = [10, 20, 30, 40];
    println!("arr[2] = {}", arr[2]);

    let mut nums = vec![1, 2, 3];
    nums.push(4);
    println!("nums = {:?}", nums);

    // 6. If / Else
    if age > 18 {
        println!("Adult");
    } else {
        println!("Minor");
    }

    // 7. Loops
    for n in 1..5 {
        println!("n = {}", n);
    }

    let mut count = 0;
    while count < 3 {
        println!("count = {}", count);
        count += 1;
    }

    // 8. Functions
    greet("Rust learner");
    let sum = add(10, 20);
    println!("10 + 20 = {}", sum);

    // 9. Tuples
    let person = ("Averi", 22, "Engineer");
    println!("Name: {}, Age: {}, Role: {}", person.0, person.1, person.2);

    // 10. Structs
    let student = Student {
        name: String::from("Alice"),
        grade: 95,
    };
    println!("{} scored {}", student.name, student.grade);

    // 11. Enums
    let color = Color::Red;
    print_color(color);

    // 12. Match (like switch)
    let day = 3;
    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        _ => println!("Other day"),
    }

    // 13. Option & Pattern Matching
    let maybe_number = Some(42);
    match maybe_number {
        Some(n) => println!("We got a number: {}", n),
        None => println!("No number found"),
    }

    // 14. Result for Error Handling
    let res = divide(10, 0);
    match res {
        Ok(val) => println!("Result = {}", val),
        Err(err) => println!("Error: {}", err),
    }

    // 15. Ownership & Borrowing
    let s1 = String::from("Hello");
    let s2 = s1.clone(); // clone instead of move
    println!("s1 = {}, s2 = {}", s1, s2);

    // 16. References
    let n = 5;
    print_ref(&n);
    println!("n is still usable = {}", n);
}

// Function definitions
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Struct example
struct Student {
    name: String,
    grade: i32,
}

// Enum example
enum Color {
    Red,
    Green,
    Blue,
}

fn print_color(c: Color) {
    match c {
        Color::Red => println!("Color: Red"),
        Color::Green => println!("Color: Green"),
        Color::Blue => println!("Color: Blue"),
    }
}

// Result type example
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

// Reference example
fn print_ref(x: &i32) {
    println!("Got a reference to {}", x);
}