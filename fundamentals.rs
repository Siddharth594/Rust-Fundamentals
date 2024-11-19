// Variables and Mutability
fn variables_and_mutability() {
    let x = 5; // Immutable
    let mut y = 10; // Mutable
    y += 5;
    println!("x: {}, y: {}", x, y);
}

// Data Types
fn data_types() {
    let integer: i32 = 10;
    let float: f64 = 10.5;
    let boolean: bool = true;
    let character: char = 'R';
    let tuple: (i32, f64, char) = (42, 6.9, 'Z');
    let array: [i32; 3] = [1, 2, 3];

    println!("Tuple: {:?}, Array: {:?}", tuple, array);
}

// Ownership and Borrowing
fn ownership_and_borrowing() {
    let s1 = String::from("hello");
    let s2 = &s1; // Borrowing
    println!("Borrowed: {}", s2);

    let s3 = s1; // Ownership moved
    println!("Moved: {}", s3);
}

// Functions
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn functions() {
    greet("Rustacean");
}

// Control Flow
fn control_flow() {
    let number = 6;

    if number % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }

    for i in 1..=5 {
        println!("Loop count: {}", i);
    }
}

// Structs and Enums
struct User {
    username: String,
    email: String,
    age: u8,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn structs_and_enums() {
    let user = User {
        username: String::from("shazil"),
        email: String::from("shazil@example.com"),
        age: 25,
    };

    let dir = Direction::Up;

    match dir {
        Direction::Up => println!("Going up"),
        Direction::Down => println!("Going down"),
        Direction::Left => println!("Going left"),
        Direction::Right => println!("Going right"),
    }

    println!("User: {}, Email: {}", user.username, user.email);
}

// Error Handling
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("Division by zero"))
    } else {
        Ok(a / b)
    }
}

fn error_handling() {
    match divide(10, 0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    println!("--- Variables and Mutability ---");
    variables_and_mutability();

    println!("\n--- Data Types ---");
    data_types();

    println!("\n--- Ownership and Borrowing ---");
    ownership_and_borrowing();

    println!("\n--- Functions ---");
    functions();

    println!("\n--- Control Flow ---");
    control_flow();

    println!("\n--- Structs and Enums ---");
    structs_and_enums();

    println!("\n--- Error Handling ---");
    error_handling();
}
