use std::collections::HashMap;
fn main() {
    let a: MyEnum = MyEnum::A;
    let b: MyEnum = MyEnum::B(96);
    let c: MyEnum = MyEnum::C{x: 96, y: 69};
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", c);

    //vector is a dynamic array in which we can push and remove elements
    let mut vec: Vec<i64> = vec![0,1,2,3]; 
    println!("{}", vec[0]);
    println!("{}", vec.len());
    vec.push(69);
    vec.remove(0);
    println!("{:?}", vec);

    // Hash maps
    let mut hash_map = HashMap::new();  
    hash_map.insert(0, "Hi");
    hash_map.insert(1, "My name is Krishang"); 
    println!("{:?}", hash_map); 
    match hash_map.get(&0) {
        Some(str) => println!("{}", str),
        None => println!("Doesnot exist in the map!")
    }
    match hash_map.get(&2) {
        Some(str) => println!("{}", str),
        _ => println!("Doesnot exist in the map!")
    }
    hash_map.remove(&1);
    println!("{:?}", hash_map); 

    //The usize type in Rust is an unsigned integer type 
    //used primarily for indexing collections and representing sizes.

    let mut hash_map1: HashMap<usize, Value> = HashMap::new();
    hash_map1.insert(0, Value::Str("Hi"));
    hash_map1.insert(1, Value::Number(456)); 
    println!("{:?}", hash_map1); 
    match hash_map1.get(&1) {
        Some(Value::Str(str)) => println!("{}", str),
        Some(Value::Number(number)) => println!("{}", number),
        None => println!("Does not exist in the map!"), 
    }
    match hash_map1.get(&2) {
        Some(Value::Str(str)) => println!("{}", str),
        Some(Value::Number(number)) => println!("{}", number),
        _ => println!("Does not exist in the map!"), 
    }
    hash_map1.remove(&0);
    println!("{:?}", hash_map1); 
}

#[derive(Debug)]
enum MyEnum {
    A,
    B(i32),
    C{x: u32, y: u32}
}
#[derive(Debug)]
enum Value {
    Str(&'static str),
    Number(u64),
}