

extern crate xmachine;
use xmachine::{Machine, Value};


fn to_num(xasm: &mut Machine) {
    let number = xasm.pop().unwrap();
    let add_one = |xasm: &mut Machine| {
        let int = xasm.pop().unwrap().to_string().parse::<i32>().unwrap();
        xasm.push(Value::number(int + 1));
    };

    xasm.push(Value::number(0));
    xasm.push(Value::function(add_one, &xasm));
    xasm.push(number);
    xasm.call();
    xasm.call();
}


fn print(xasm: &mut Machine) {
    if let Some(string) = xasm.pop() {
        print!("{}", string);
    }
}

fn println(xasm: &mut Machine) {
    if let Some(string) = xasm.pop() {
        println!("{}", string);
    }
}

fn add(xasm: &mut Machine) {
    let first = xasm.pop();
    let second = xasm.pop();

    if let (Some(m), Some(n)) = (first, second) {
        let m_f = m.to_string().parse::<f64>().unwrap();
        let n_f = n.to_string().parse::<f64>().unwrap();

        xasm.push(
            Value::number(m_f + n_f)
        );
    }
}

fn sub(xasm: &mut Machine) {
    let first = xasm.pop();
    let second = xasm.pop();

    if let (Some(m), Some(n)) = (first, second) {
        let m_f = m.to_string().parse::<f64>().unwrap();
        let n_f = n.to_string().parse::<f64>().unwrap();

        xasm.push(
            Value::number(m_f - n_f)
        );
    }
}


fn main() {
	let mut xasm = Machine::new();
	
    xasm.push(Value::function(println, &xasm));
 xasm.copy();
xasm.push(Value::string("println"));
xasm.store();
 xasm.push(Value::function(to_num, &xasm));
 xasm.copy();
xasm.push(Value::string("to_num"));
xasm.store();
 xasm.push(Value::function(print, &xasm));
 xasm.copy();
xasm.push(Value::string("print"));
xasm.store();
 xasm.push(Value::function(sub, &xasm));
 xasm.copy();
xasm.push(Value::string("sub"));
xasm.store();
 xasm.push(Value::function(add, &xasm));
 xasm.copy();
xasm.push(Value::string("add"));
xasm.store();
 xasm.push(Value::function(|xasm: &mut Machine| {xasm.push(Value::string("a"));
xasm.store();
 xasm.push(Value::function(|xasm: &mut Machine| {xasm.push(Value::string("b"));
xasm.store();
 xasm.push(Value::string("a"));
 xasm.load();
}, &xasm));
}, &xasm));
 xasm.copy();
xasm.push(Value::string("True"));
xasm.store();
 xasm.push(Value::function(|xasm: &mut Machine| {xasm.push(Value::string("a"));
xasm.store();
 xasm.push(Value::function(|xasm: &mut Machine| {xasm.push(Value::string("b"));
xasm.store();
 xasm.push(Value::string("b"));
 xasm.load();
}, &xasm));
}, &xasm));
 xasm.copy();
xasm.push(Value::string("False"));
xasm.store();
 xasm.push(Value::function(|xasm: &mut Machine| {xasm.push(Value::string("a"));
xasm.store();
 xasm.push(Value::function(|xasm: &mut Machine| {xasm.push(Value::string("b"));
xasm.store();
 xasm.push(Value::string("a"));
 xasm.load();
 xasm.push(Value::string("b"));
 xasm.load();
 xasm.push(Value::string("a"));
 xasm.load();
xasm.call();
xasm.call();
}, &xasm));
}, &xasm));
 xasm.copy();
xasm.push(Value::string("And"));
xasm.store();
 xasm.push(Value::function(|xasm: &mut Machine| {xasm.push(Value::string("a"));
xasm.store();
 xasm.push(Value::function(|xasm: &mut Machine| {xasm.push(Value::string("b"));
xasm.store();
 xasm.push(Value::string("b"));
 xasm.load();
 xasm.push(Value::string("a"));
 xasm.load();
 xasm.push(Value::string("a"));
 xasm.load();
xasm.call();
xasm.call();
}, &xasm));
}, &xasm));
 xasm.copy();
xasm.push(Value::string("Or"));
xasm.store();
 xasm.push(Value::function(|xasm: &mut Machine| {xasm.push(Value::string("a"));
xasm.store();
 xasm.push(Value::string("True"));
 xasm.load();
 xasm.push(Value::string("False"));
 xasm.load();
 xasm.push(Value::string("a"));
 xasm.load();
xasm.call();
}, &xasm));
 xasm.copy();
xasm.push(Value::string("Not"));
xasm.store();
 xasm.push(Value::function(|xasm: &mut Machine| {xasm.push(Value::string("c"));
xasm.store();
 xasm.push(Value::function(|xasm: &mut Machine| {xasm.push(Value::string("a"));
xasm.store();
 xasm.push(Value::function(|xasm: &mut Machine| {xasm.push(Value::string("b"));
xasm.store();
 xasm.push(Value::string("b"));
 xasm.load();
 xasm.push(Value::string("a"));
 xasm.load();
 xasm.push(Value::string("c"));
 xasm.load();
xasm.call();
xasm.call();
}, &xasm));
}, &xasm));
}, &xasm));
 xasm.copy();
xasm.push(Value::string("If"));
xasm.store();
 xasm.push(Value::number(10000));
 xasm.copy();
xasm.push(Value::string("n"));
xasm.store();
 xasm.push(Value::function(|xasm: &mut Machine| {xasm.push(Value::string("b"));
 xasm.push(Value::string("a"));
 xasm.push(Value::string("False"));
 xasm.load();
 xasm.push(Value::string("True"));
 xasm.load();
 xasm.push(Value::string("And"));
 xasm.load();
xasm.call();
xasm.call();
 xasm.push(Value::string("If"));
 xasm.load();
xasm.call();
xasm.call();
xasm.call();
 xasm.push(Value::string("println"));
 xasm.load();
xasm.call();
 xasm.push(Value::number(1));
 xasm.push(Value::string("n"));
 xasm.load();
 xasm.push(Value::string("sub"));
 xasm.load();
xasm.call();
 xasm.copy();
xasm.push(Value::string("n"));
xasm.store();
}, &xasm));
xasm.push(Value::function(|xasm: &mut Machine| {xasm.push(Value::string("n"));
 xasm.load();
}, &xasm));
xasm.while_loop();


    println!("{}", xasm);
}
