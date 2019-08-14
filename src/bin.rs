extern crate xmachine;
use xmachine::{Machine, Value, Ref};

fn main() {
    let mut m = Machine::new();


    m.push(Ref::new(Value::Number(5.0)));
    m.push(Ref::new(Value::String(String::from("test"))));
    m.store();

    m.push(Ref::new(Value::String(String::from("wow"))));
    m.push(Ref::new(Value::String(String::from("ASSIGN TO REFERENCE WORKS!"))));
    m.push(Ref::new(Value::String(String::from("test"))));
    m.load();
    m.assign();
    m.push(Ref::new(Value::String(String::from("test"))));
    m.load();
    m.assign();

    m.push(Ref::new(Value::String(String::from("test"))));
    m.load();



    println!("{}", m);
}