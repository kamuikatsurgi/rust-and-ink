//=========== Variables ===============
// fn main() {
//     let mut x = 4;
//     println!("x is {}", x);

//     {
//         let x = x - 2;
//         println!("x is {}", x);
//     } // This is another scope which is different than main function scope

//     x = x + 1;
//     println!("x is {}", x);

//     let y = 6;
//     println!("y is {}", y); 

//     let y = 7;
//     println!("y is {}", y);    

//     let x = "hello world";
//     println!("{}", x);

//     const SECONDS_IN_MINUTE: u8 = 60; // constant value can not be change and type should be defined 
//     println!("{}", SECONDS_IN_MINUTE);
// }
//=====================================

//=========== Data Types ==============

// fn main() {

//     let x: i8 = -96; //default int type in rust is i32, i8 -2^7 to 2^7-1(-128 to 127)
//     let _x: u8 = 96;  //u8 0 to 2^8 -1(0 to 255)
//     println!("{} {}", x, _x);
//     let _floating_point: f32 = 10.9; //32 bit floating point type variable
//     let _true_or_false: bool = true; //boolean
//     let _letter: char = 'a'; // can you numbers, symbols, characters and anything on the keyboard

//     // tuples
//     let tup: (i32, bool, char) = (1, true, '@'); 
//     let mut tup2: (i8, bool, char) = (1, false, '8');
//     println!("{}", tup.0);
//     tup2.1 = true;
//     println!("{}", tup2.1);
    
//     // arrays
//     let mut arr: [i32; 5] = [0,-1,2,3,4];
//     let mut _arr: [i32; 0] = [];
//     arr[4] = 10;
//     println!("{}", arr[4]);
// }

//=====================================

//=========== User Input ==============

// use std::io;

// //:: is sommeting like going from std crate/package/library to io method

// fn main() {
//     //:: grabs the new functions from the String data type
//     let mut input = String::new();
//     // if we give input it will not change the variable but change its copy
//     // so we need to give a refrence to the variable by giving &input 
//     // but &input will be immutable so we nee to add mut
//     io::stdin().read_line(&mut input).expect("Failed to read the string");
    
//     println!("User typed: {}", input);

// }

//=====================================

//============ Arithmetic and Typecasting ==============

// use std::io;
// fn main() {
//     let x = 1270 as i64;
//     let y = 10 as i32;
//     let z = x / (y as i64);
//     println!("{}", z);

//     let a = i32::MAX; // to get the max value for the 32 bit integer
//     println!("{}", a);

//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("Failed to read line");

//     let int_input: i64 = input.trim().parse().unwrap();
//     println!("{}", int_input + 10);
// }

//======================================================