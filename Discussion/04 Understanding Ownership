// 4. Ownership
// (1) What is Ownership?
// Ownership: a set of rules that govern how a Rust program manages memory
// Memory is managed through a system of ownership with a set of rules that the compiler checks.

// The Stack and the Heap
// 시스템 프로그램 언어에서는 stack, heap의 개념이 활용됨.
// Ownership 또한 stack, heap을 알아야 이해 가능
// stack, heap: parts of memory available at runtime. # slide 1
// https://youtu.be/8aH54mBTVLQ
// request a certain amount of space -> memroy allocator finds empty spot & returns pointer
// allocationg velocity: stack > heap
// Processors: stack < heap
// The main purpose of ownership: manage heap data can help explain why it works the way it does.

// Ownership Rules
// 1) Each value in Rust has an owner.
// 2) There can only be one owner at a time.
// 3) When the owner goes out of scope, the value will be dropped.

// example - String
// Cons of string literal
// 1) immutable
// 2) not every string value can be known

/*
fn main () {
    // String::from -> string 생성
    // s is immutable.
    let s: String = String::from("hello");

    // t is mutable.
    let mut t: String = String::from("hello");
    t.push_str(", world!");
    println!("{s}");
    println!("{t}");
}
*/

// Memory and Allocation
// In order to support a mutable, we need to allocate an amount of memory on the heap.
// 1) At runtime, the memory must be requested from the memory allocater.
// String:: from -> request the memory it needs. (universal)
// 2) We need a way of returning this memory to the allocator when we're done with our String.
// Other language -> Garbage Collector, free
// Rust -> out of scope, drop
// drop : automatically called at the closing scope, similar to RAII patterns
/*
{
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s
}                                  // this scope is now over, and s is no
*/

// Variables and Data Interacting with Move
/* 
fn main () {
    // y know the size of data
    // Call by value
    let x: i32 = 5;
    let y: i32 = x;

    // s2 copies the pointer of s1[0]
    // Call by reference (but not same)
    let s1: String = String::from("hello");
    // let s2: String = s1;
    let s2: String = s1.clone();

    // If s1, s2 go out of scope, they will both try to free the same memory
    // s1 invalidate -> s2 copies the pointer of s1[0]
    // we can use clone to copy String.
    println!("{}", x);
    println!("{}", y);
    println!("{}, world!", s1);
    println!("{}, world!", s2);
}
*/

// Stack-Only data: Copy
// Copy: Place on types that are stored on the stack -> variables don't move, just copied
// Still valid after the other variable is go out of scope.
// int, bool, float, char, tuple (only one type) ex. (i32, i32)
/* 
fn main () {
    let mut x: i32 = 5;
    makes_copy(x);
    x = 7;
    let mut y: i32 = x;
    println!("{}", y);
    x = 9;
    println!("{}", y);
}

fn makes_copy(input_int: i32) {
    //input_int copy x
    println!("{}", input_int);
}
*/

// Ownership and Functions
/* 
fn main() {
    let mut s: String = String::from("hello");
    // takes_ownership(s);
    println!("{}", takes_ownership(s.clone()));
    println!("{}", s);
    s = takes_ownership(s.clone());
    println!("{}", s);

    let x = 5;
    makes_copy(x);    
}

fn takes_ownership(mut some_string: String) -> String {
    // Move ownership of some_string
    println!("{}", some_string);
    some_string.push_str(", world!");
    println!("{}", some_string);
    some_string
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
*/

// Return Values and Scope
/* 
fn main() {
    // Move ownership of some_string to s1
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    // let s3 = takes_and_gives_back(s2);
    let s3 = takes_and_gives_back(s2.clone());

    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
*/

// Rust can treat a value using ownership (surrent chapter) or reference(next chapter)

// (2) References and Borrowing
// What is Reference?
/* 
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    // Pointer of s is pointing the s1[0]
    // Not Owning, just refer
    s.len()
}
*/

// Mutable References
/* 
fn main() {
    let mut s: String = String::from("hello");
    println!("{}", s);
    // default is immutable
    change(&mut s);
    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
*/

// We can create only one mutable reference
// pros: protect data races at compile time
// cons: very controlled fashion
/* 
fn main() {
    let mut s: String = String::from("hello");

    // write same data
    // let r1: &mut String = &mut s;
    // let r2: &mut String = &mut s;

    // read data
    let r1: &String = &s;
    let r2: &String = &s;
    // can write and read at the same time
    // let r3: &mut String = &mut s;

    println!("{}, {}", r1, r2);
}
*/
// Data Race Condition
// 1) two ore more pointers access the same data at the same time
// 2) At least one of the pointers is being used to write to the data
// 3) There’s no mechanism being used to synchronize access to the data
// These cause undefined behavior, difficult to diagnose and fix

// We have to discriminate read refs and the write ref
/* 
fn main() {
    let mut s: String = String::from("hello");

    let r1: &String = &s;
    let r2: &String = &s;
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3: &mut String = &mut s;
    println!("{}", r3);
}
*/

// Dangling References
// C++ -> The dangling pointer is dangerous!
// Rust -> The dangling reference is dangerous!
/* 
fn main() {
    let reference_to_nothing: &String = dangle();
    println!("{}", reference_to_nothing);
}

fn dangle() -> &String {
    // s goes out of scpoe when allocated to reference_to_nothing
    let s: String = String::from("hello");
    &s
}
*/
// Sol) Directly use string
/* 
fn main() {
    let reference_to_nothing: String = no_dangle();
    println!("{}", reference_to_nothing);
}

fn no_dangle() -> String {
    let s: String = String::from("hello");
    s
}
*/

// The rules of References
// 1) You can have either one mutable reference or any number of immutable references.
// 2) References must always be valid. (not local)

// (3) The Slice Type
// What is Slice?
// Slice: you can reference a contiguous sequence of elementsin a collection
// No whole collection
// No owndership

// Example (first_word function)
// input: a string of words separated by spaces
// output: the first word
/* 
fn main() {
    let mut input: String = String::from("hello, world!");
    let first_word_len: usize = first_word(&input);
    println!("{}", first_word_len);
    input.clear();
}

fn first_word(s: &String) -> usize {
    // Convert string to array
    let bytes: &[u8] = s.as_bytes();

    // iter(): returns each element in a collectiom
    // enumerate(): wraps the result of iter using tuple
    // ex) [I, , a, m] -> [(0, I), (1,  ), (2, a), (3, m)]
    for (i, &item) in bytes.iter().enumerate() {
        // If there is space, return the index of space
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
*/

// s.len is not meaningful to String (made by array)
// If s change, value of word is wrong.
// When check second word, we need more index that is not related to our needs.
// We can solve it using string slices

// String Slices
// String Slice: a reference to part of a String
// [start_index..end_index]: end_index is one more than the last position in the slice
// [..2] == [0..2], [5..] == [5..s.len()], [..] == [0..s.len()]
// ex) s = hello word -> hello = &s[0..5]; world = &s[6..11];
// Caution: If you use multibyte character, there will be error

// Remake of first_word Function
/* 
fn main() {
    let mut input: String = String::from("hello, world!");
    let first_word_len: &str = first_word(&input);
    println!("{}", first_word_len);
    input.clear();
    // If we change s, first_word_len should be changed too.
    // There will be an error.
    // input is still referenced. -> can not clear the input
    // println!("{}", first_word_len);
}

fn first_word(s: &String) -> &str {
    let bytes: &[u8] = s.as_bytes();

    // If we find space, function return the slice of a first word
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    // If we can't find space, function return the whole string.
    &s[..]
}
*/

// We can slice the part of the string
// ex. word = first_word(&my_string[0..6]);

// Other Slices
/* 
fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let slice: &[i32] = &a[1..3];
    // assert_eq(a): check whether a equal b
    // Use when we need value test
    assert_eq!(slice, &[2, 3]);
    for (i, items) in slice.iter().enumerate() {
        println!("{}, {}", i, items);
    }
}
*/
