use core::panic;
use std::vec;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

#[derive(Debug)]
struct Foo {
    value: usize
}

struct Person<'a> {
    name: &'a str,
    age: u32,
}

impl<'a> Person<'a> {
    fn new(name: &'a str, age: u32) -> Person<'a> {
        Person { name, age }
    }

    fn get_name(&self) -> &'a str {
        self.name
    }

    fn get_age(&self) -> u32 {
        self.age
    }
}

struct FileHandler {
    file: File,
    is_open: bool,
}

impl FileHandler {
    fn open_file(filename: &str) -> io::Result<FileHandler> {
        // let file = File::open(filename)?;
        let file = OpenOptions::new().read(true).write(true).open(filename).unwrap();
        Ok(FileHandler {
            file,
            is_open: true,
        })
    }

    fn read_file(&mut self, buffer: &mut String) -> io::Result<usize> {
        if !self.is_open {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "File handler is not open.",
            ));
        }
        self.file.read_to_string(buffer)
    }

    fn write_file(&mut self, data: &[u8]) -> io::Result<usize> {
        if !self.is_open {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "File handler is not open.",
            ));
        }
        self.file.write(data)
    }
}

fn main() {
    ownership(); // J
    borrowing(); // M
    mutable_borrowing(); // M
    scoping(); // J
    ownership_functions(); // J
    functions_references(); // M
    copy_variables(); // M


    lifetimes(); // J
    list_example(); // M
    filehandler_example(); // J
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
    println!("Mutable origin: {}", mutable);
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

fn take_ownership(value: String) -> String {
    println!("Taking ownership: {}", value);
    value // Ownership is returned from the function
}

fn functions_references() {
    // References in Functions
    let original = String::from("Original");
    let length = calculate_length(&original); // Passes a reference to the function
    println!("Original: {}, Length: {}", original, length);
}

fn calculate_length(s: &String) -> usize {
    s.len() // References can be used for read-only operations
}

fn copy_variables() {
    let num1 = 42;
    let num2 = num1.clone();

    println!("num1: {}", num1);
    println!("num2: {}", num2);

    let name1 = String::from("Alice");
    let name2 = name1.clone();

    println!("name1: {}", name1);
    println!("name2: {}", name2);

    #[derive(Clone)]
    struct Point {
        x: i32,
        y: i32,
    }

    let point1 = Point { x: 2, y: 5 };
    let point2 = point1.clone();

    println!("point1: ({}, {})", point1.x, point1.y);
    println!("point2: ({}, {})", point2.x, point2.y);
}

fn lifetimes() {
    let name = String::from("Alice");
    let person_name;
    let person_age;
    
    {
        let person = Person::new(&name, 25);
        // drop(name);
        person_name = person.get_name();
        person_age = person.get_age()
    }

    // Using person_name after person is dropped will cause a potential memory leak
    println!("Person's name: {} , age: {}", person_name, person_age);

    let data = Box::new(42);

    let data_ptr = &*data as *const i32;

    // Simulating a memory leak by not deallocating the memory
    // Uncomment the following line to trigger a crash due to accessing invalid memory
    // drop(data);

    // Attempting to access the memory through the dangling pointer
    unsafe {
        println!("Value: {}", *data_ptr);
    }
}

fn list_example() {
    let vector = vec![Foo{value: 1}, Foo{value: 2}, Foo{value: 3}];

    reverse_and_print(&vector);
    reverse_and_print(&vector);
}

fn reverse_and_print(foo: &Vec<Foo>) {
    // foo.reverse();
    
    // for f in foo.iter() {
    for f in foo.iter().rev() {
        println!("{:?}", f.value);
    }
}

fn filehandler_example() {
    let filesystem: FileHandler;
    {
        filesystem = match filesystem_fn() {
            Ok(filehandler) => filehandler,
            Err(error) => panic!("Problem with filesystem: {:?}", error),
        };
    }
    drop(filesystem);
}

fn filesystem_fn() -> io::Result<FileHandler> {
    let mut file_handler = FileHandler::open_file("lorem.txt")?;
    let mut buffer = String::new();
    let bytes_read = file_handler.read_file(&mut buffer)?;
    println!("Read {} bytes: {:?}", bytes_read, &buffer[..bytes_read]);

    let data: &[u8; 14] = b"\nHello, World!";
    let bytes_written = file_handler.write_file(data)?;
    println!("Written {} bytes", bytes_written);

    Ok(file_handler)
}