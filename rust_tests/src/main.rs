use std::env;
use std::mem;
fn main() {
    println!("Rust program running...");
    cli();
    //run whatever function here
}
//cli (command line interfacd)
pub fn cli() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Brad";
    let status = "100%";
    //println!("Command: {:?}",command)
    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        println!("Status is {}", status)
    } else {
        println!("That is not a valid command");
    }
}
//enums
enum Movement {
    //variants
    /*Up,
    Down,*/
    Left,
    //Right
}
fn move_avatar(m: Movement) {
    //Perform action depending on info
    match m {
        Movement::Left => println!("Avatar moving left"),
        //other things...
    }
}
pub fn enums() {
    //enums are types that have a few definite values
    let avatar1 = Movement::Left;
    move_avatar(avatar1);
}
//structs
//traditional struct
/*struct Color {
    red: u8,
    green: u8,
    blue: u8,
}*/
// tuple Struct
//struct Color(u8,u8,u8);

struct Person {
    first_name: String,
    last_name: String,
}
impl Person {
    //construct Person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}
pub fn structs() {
    /* let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    c.red = 200;
    println!("Color: {} {} {} ", c.red, c.green, c.blue);*/
    /* let mut c = Color(255,0,0);
    c.0 = 200;
    println!("Color: {} {} {}",c.0,c.1,c.2);*/
    let mut p = Person::new("John", "Doe");
    println!("Person {}", p.full_name());
    p.set_last_name("Tix");
    println!("Person {}", p.full_name());
    println!("Person Tuple {:?}", p.to_tuple());
}
//functions

pub fn functions() {
    //used to store blocks of code
    greeting("Hello", "Jane");
    //bind function values to variables
    let get_sum = add(5, 5);
    println!("Sum : {}", get_sum);
    let n3 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C Sum: {}", add_nums(3, 3));
}
fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}
fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
pub fn loops() {
    //let mut count = 0;
    //infinite loop ;|
    /*loop {
      count+=1;
      println!("number: {}", count);
      if count == 20 {
          break;
      }
    }*/
    //while loop (FizzBuzz)
    /*while count <= 100 {
        if count % 15 == 0 {
            println!("fizzbuzz");
        } else if count %3 == 0{
            println!("fizz");
        } else if count % 5 == 0{
            println!("buzz");
        } else {
            println!("{}",count);
        }
        count+= 1;
    }*/
    //for range
    for x in 0..100 {
        if x % 15 == 0 {
            println!("fizzbuzz");
        } else if x % 3 == 0 {
            println!("fizz");
        } else if x % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", x);
        }
    }
}
pub fn array() {
    let mut numbers: [i64; 5] = [1, 2, 3, 4, 5];
    numbers[2] = 20;
    println!("{:?}", numbers);
    println!("Array length : {}", numbers.len());
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));
    let slice: &[i64] = &numbers[0..2];
    println!("Slice: {:?}", slice)
}
pub fn condition() {
    let age: u8 = 18;
    let check_id: bool = false;
    let knows_person_of_age: bool = true;
    //if/else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: What would you like to drink?")
    } else if age < 21 && check_id {
        println!("Bartender: Sorry, you have to leave.")
    } else {
        println!("Bartender: I'll need to see your ID")
    }
    //shorthand if
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of Age:{}", is_of_age);
}
pub fn vector() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];
    numbers[2] = 20;
    //adding to vector
    numbers.push(5);
    numbers.push(6);
    println!("{:?}", numbers);
    println!("Vector length : {}", numbers.len());
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
    for x in numbers.iter() {
        println!("Number: {}", x);
    }
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers);
}
