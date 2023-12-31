use std::collections::HashMap;

// 变量
let mut a:i32 = 1;
let mut s:&str = "hello world!";
let mut b:bool = true;
let mut ss:String = "hello world!".to_string();
let mut v:Vec<i32> = vec![1, 2, 3];
let mut m:HashMap<i32, i32> = HashMap::new();

// 常量
let b:i32 = 2;

// 函数
fn add(a:i32, b:i32) -> i32 {
    return a + b;
}

add(1, 2);

// trait
trait Test {
    fn test(&self);
}

// struct
#[derive(Debug)]
struct Person {
    name: String,
    age: i32
}

// impl
impl Test for Person {
    fn test(&self) {
        println!("{}", self.name);
    }
}

let person = Person{
    name: "John".to_string(),
    age: 30
};
person.test();

// enum
enum Color {
    Red,
    Green,
    Blue
}