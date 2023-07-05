fn main() {
    ownership();
    borrowing();
    mutable_borrowing();
    scoping();
    ownership_functions();
    functions_references();
}


fn ownership() {
    // Ownership and Transfer
    let owner = String::from("Hello"); // owner takes ownership of the String
    // println!("{}", owner);
    let transferred: String = owner; // ownership is transferred to transferred
    println!("{}", transferred);
    // println!("{}", owner); // This line would result in a compile-time error
}

fn borrowing() {
    let owner = String::from("Hello"); // owner takes ownership of the String
    // Borrowing
    let borrowed = &owner; // borrowed references transferred
    println!("Borrowed: {}", borrowed); // borrowed can be used
    println!("Owner: {}", owner); // And the owner is still there
}

fn mutable_borrowing() {
    // Mutable Borrowing
    let mut mutable = String::from("World");
    let borrowed_mut = &mut mutable; // mutable borrowing
    borrowed_mut.push_str(", Rust!"); // borrowed_mut can modify mutable
    println!("Mutable Borrowed: {}", borrowed_mut);
}

fn scoping() {
    // Scope and Drop
    {
        let scoped = String::from("Scoped"); // scoped takes ownership
        println!("Scoped: {}", scoped); // scoped can be used
    } // scoped goes out of scope and is dropped
    // println!("{}", scoped) // Trying to use scoped leads to an error
}

fn ownership_functions() {
    let owner = String::from("Hello"); // owner takes ownership of the String
    // Ownership and Functions
    let new = take_ownership(owner); // transferred ownership to the function
    println!("Owned: {}", new);
    // println!("Owned: {}", owner);
}

fn functions_references() {
    // References in Functions
    let original = String::from("Original");
    let length = calculate_length(&original); // Passes a reference to the function
    println!("Original: {}, Length: {}", original, length);
}

fn take_ownership(value: String) -> String {
    println!("Taking ownership: {}", value);
    value // Ownership is returned from the function
}

fn calculate_length(s: &String) -> usize {
    s.len() // References can be used for read-only operations
}
