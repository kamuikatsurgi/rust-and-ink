use std::io;

fn main() {
    // =========== Variables ===============   
    let mut x = 4;
    println!("x is {}", x);
    {
        let x = x - 2;
        println!("x is {}", x);
    } // This is another scope which is different than main function scope
    x = x + 1;
    println!("x is {}", x);
    let y = 6;
    println!("y is {}", y); 
    let y = 7;
    println!("y is {}", y);    
    let x = "hello world";
    println!("{}", x);
    const SECONDS_IN_MINUTE: u8 = 60; // constant value can not be change and type should be defined 
    println!("{}", SECONDS_IN_MINUTE);
    //=====================================

    //=========== Data Types ==============
    let x: i8 = -96; //default int type in rust is i32, i8 -2^7 to 2^7-1(-128 to 127)
    let _x: u8 = 96;  //u8 0 to 2^8 -1(0 to 255)
    println!("{} {}", x, _x);
    let _floating_point: f32 = 10.9; //32 bit floating point type variable
    let _true_or_false: bool = true; //boolean
    let _letter: char = 'a'; // can you numbers, symbols, characters and anything on the keyboard

    // tuples
    let tup: (i32, bool, char) = (1, true, '@'); 
    let mut tup2: (i8, bool, char) = (1, false, '8');
    println!("{}", tup.0);
    tup2.1 = true;
    println!("{}", tup2.1);
    
    // arrays
    let mut arr: [i32; 5] = [0,-1,2,3,4];
    let mut _arr: [i32; 0] = [];
    arr[4] = 10;
    println!("{}", arr[4]);
    //=====================================

    //=========== User Input ==============
    // :: is sommeting like going from std crate/package/library to io method
    // :: grabs the new functions from the String data type
    let mut input = String::new();
    // if we give input it will not change the variable but change its copy
    // so we need to give a refrence to the variable by giving &input 
    // but &input will be immutable so we nee to add mut
    io::stdin().read_line(&mut input).expect("Failed to read the string");
    println!("User typed: {}", input);
    //=====================================

    //===== Arithmetic and Typecasting ====
    let x = 1270 as i64;
    let y = 10 as i32;
    let z = x / (y as i64);
    println!("{}", z);

    let a = i32::MAX; // to get the max value for the 32 bit integer
    println!("{}", a);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let int_input: i64 = input.trim().parse().unwrap();
    println!("{}", int_input + 10);
    //=====================================

    //=== Conditions and Control Flow =====
    // && (and)
    // || (or)
    // ! (not)

    let mut food_item = String::new();
    io::stdin().read_line(&mut food_item).expect("Failed to get input");

    let food_item = food_item.trim(); // trim is to remove the last invisible special char in the string 

    if food_item == "cookie" {
        println!("Which cookie are you eating?");
    } else if food_item == "fruit" {
        println!("You are eating a fruit")
    } else {
        println!("Enjoy eating {}", food_item);
    }
    //===================================== 

    //============= Structs ===============
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool
    }

    fn build_user(_email: String, _name: String) -> User {
        return User{
            username: _name,
            email: _email,
            sign_in_count: 1,
            active: true
        }
    }
    //add mut if you want to change the fields of this struct
    let user1 = User {
        email: String::from("shahkrishang11@gmail.com"),
        username: String::from("kamuikatsuragi"),
        sign_in_count: 1,
        active: true
    };

    println!("Name of user1 is: {}", user1.username);
    println!("Email of user1 is: {}", user1.email);
    println!("Signed count of the user1 is: {}", user1.sign_in_count);
    println!("Status of user1 is: {}", user1.active);

    let user2: User = build_user(String::from("john@email.com"),String::from("John"));
    println!("Email of user 2 is: {}", user2.email);

    struct Rectangle {
        width: u64,
        height: u64
    }

    impl Rectangle {

        fn area(&self) -> u64 {
            return self.width * self.height;
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            return self.width > other.width && self.height > other.height;
        }
    }
    
    let rect1: Rectangle = Rectangle{height: 20, width: 20};
    let rect2: Rectangle = Rectangle{height: 10, width: 10};

    println!("Area of rect1 is: {}", rect1.area());
    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    //=====================================
    
    //===== Functions and Expressions =====
    test();
    add_numbers(1,2);
    println!("{}", add_numbers_return(1, 1));
    //=====================================

}
//======== Functions and Expressions ======
fn test() {
    println!("test function called")
}    
fn add_numbers(a: u64, b: u64) {
    println!("The sum of the two numbers is: {}", a+b);
}
fn add_numbers_return(a: u64, b: u64) -> u64 {
    if a+b> 10 {
        return a+b-10;
    } else {
        return a+b;
    }
}    
//==========================================