// Chaper 3
// (1) Variables and Mutability

// immutable varialbe
/*
fn main() {
    let x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
}
*/


// mutable variable
/*
fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
}
*/

// Constants
// not default value, have to use a keyword "const"

// Shadowing
// We can declare a new variavle with the same name as a previous variable.
/* 
fn main() {
    // 1st: Bind x to 5
    let x = 5;
    // 2nd: Create new value "x" and add previous x(5) and 1
    let x = x + 1;

    {
        // 3rd: Create new value "x"(inner scope) and multiply x(6, outer scope) and 2
        let x = x * 2;
        println!("The value of x in the inner scope is {x}");
    }
    // 4th: End the shadow of inner scope and go back to outer scope x
    println!("The value of x is {x}");
}
*/

// Shadow VS mut
// 1. Using shadow, we can perform a transformation, and make the variable immutable
// 2. We can change the trpe of the value and reuse the same name.
/* 
fn main() {
    let spaces = "   "; // string
    let spaces = spaces.len(); //number

    let mut spaces = "   ";
    spaces = spaces.len();
}
*/

// (2) Data types
// data = scalar + compound

// Rust is a statically typed language.
// Rust should know what the type we use at compile time.
/* 
fn main() {
    // parse: convert a string to another type. ex) a string to a number.
    let guess : u32 = "42".parse().expect("Not a number!");
    let guess = "42".parse().expect("Not a number!");
}
*/

// Scalar types
// Represents a single value 
// ex) int, float, bool, char ...

// Integer types
// u: unsigned, i: signed
// 8-bit: 8, 16-bit: 16, 32bit: 32, 64-bit: 64, 128bit: 128, arch: size
// size: depend on the architecture og the computer (32 vs 64)
// defualt int = i32

// Integer overflow
// Unrecoverable Errors with panic
// checks got interger overflow that cause yout program to panic at runtime
// --release: do not check panic, performs two's complement wrapping
// two's complement wrapping: change big value to the maximum value the type can hold
// ex) 256 -> 0, 257 -> 1

// Overflow methods
// wrapping_*: wrap the value
// checked_*: if there is overflow: return the "None" value
// overflowing_*: return the value and a bollean indicating whether there was overflow
// saturating_*: saturate at the value's min & max

// Floating-point Types
// f64(defualt), f32
/* 
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    println!("x is {}", type_of(x));
    println!("x is {}", type_of(y));
}
*/

// Numeric Operations
/* 
fn main() {
    // addition
    let a = 5 + 10;

    // subtraction
    let b = 95.5 - 4.3;

    // multiplication
    let c = 4 * 30;

    // division
    let d = 56.7 / 32.2;
    let e = -5  / 3; // Results in -1
    let f = -5f32  / 3f32;

    // remainder
    let g = 43 % 5;
    println!("{a}, {b}, {c}, {d}, {e}, {f}, {g}");
}
*/

// The Boolean Type
// boolean = true + false (1byte)
// The main way to use bool is through conditionals. ex) if

// The Character Type
// Unicode Scalar Value
// https://html-css-js.com/html/character-codes/icons/
/* 
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_ribboned = '💝';

    println!("{c} {z} {heart_ribboned}");
}
*/

// Compound Types
// Compound types = tuples + arrays

// The Tuple type
// Grouping together a number of values with a variety of types
/* 
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
fn main() {
    let tup1: (i32, f64, u8) = (500, 6.4, 1);
    println!("{} {} {}", type_of(tup1.0), type_of(tup1.1), type_of(tup1.2));
    let tup2 = (500, 6.4, 1);
    let tup1 = (400, 3.4, 0);
    let (x, y, z) = tup2;

    println!("{} {} {}", type_of(x), type_of(y), type_of(z));
    println!("{} {} {}", type_of(tup1.0), type_of(tup1.1), type_of(tup1.2));
}
*/

// The Array Type
// Every element of an array must have the same type.
// An array isn't as flexible as the vector type
// Vector allow grow or shrink in size.
// If we know the number of elements will not need to change, arrays are mor useful.
/* 
fn main() {
   let a: [i32; 5] = [1, 2, 3, 4, 5];
   let b = [3; 5]; // b = [3, 3, 3, 3, 3];

   let first = a[0];
   let second = b[1];

   println!("{first} {second}")
}
*/
// Invalid Array Element Access
/* 
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
*/
// If user's input is more than 4, ther will be a run time error

// (3) Functions
// Convention is a snake case. ex) another_function()
// We don't need the forward declaration.
/* 
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
*/

// Parameters
// Parameters: special variables that are part of a function's signature.
// Arguments: parameters that have a concrete value.
/* 
use std::any::type_name;

fn main() {
    let num = 5;
    println!("The type of num is: {}", type_of(num));
    another_function(num);
}

// You must declare the type of each parameter!
fn another_function(x: i16) {
    println!("The value of x is: {x}");
    println!("The type of x is: {}", type_of(x));
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
*/

// Statements and Expressions
// Statements: perform some action ex) function definitions
// Statement do not return a value.
// ex) let x = (let y = 6); -> error
// Expressions: evaluate to a resultant value
// ex) let y = {let x=3; x+1};
// x+1 -> expression, x+1; -> statement

// Functions with return Values
// We must declare their type uisng arrow (->)
// no arrow, no return
// return value: value of the final expression
/* 
fn main() {
    let x = five();
    let y = plus_one(x);

    println!("The value of y is: {y}");
}

fn five() -> i32 {
    5
}


fn plus_one(num: i32) -> i32 {
    num + 1
}
*/

// (5) Control Flow
// if
/* 
fn main() {
    let number = 3;
    // not automatically try to convert non-bool to bool
    if number {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
*/

// else if
// If you have more than one elseif, you can refactor your code. ex) match

// Using if in a let Statement
/*
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
 */
// Arms should have the same type values.
// Because Rust have to know the type of variables at compile time.
/* 
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
}
*/

// Repetiton with Loops
// loop, while, for

// loop
// While a user prints ctrl+c or interrupts, a block of code executed forever.
// We can use break to break out of a loop, continue to skip over.
// Using loopand counter, we can check whether a thread has completed its job.
/* 
fn main() {
    let mut counter = 0;

    // We can labels to check what the job the loop execute.
    let result = 'counting_up: loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
*/

// while
/* 
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
*/

//for
// You wouldn't need to remeber to change any other code if you changed the number of values in the array.
/* 
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
*/
// We can use a for loop when a number of times id fixed.
/* 
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
*/
/*
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
*/
