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

fn main() {
    let mut xasm = Machine::new();

    xasm.push(Value::function(to_num, &xasm));
    xasm.push(Value::string("to_num"));
    xasm.store();

    xasm.push(Value::function(
        |xasm: &mut Machine| {
            xasm.push(Value::string("a"));
            xasm.store();
            xasm.push(Value::function(
                |xasm: &mut Machine| {
                    xasm.push(Value::string("b"));
                    xasm.store();
                    xasm.push(Value::string("a"));
                    xasm.load();
                },
                &xasm,
            ));
        },
        &xasm,
    ));
    xasm.copy();
    xasm.push(Value::string("True"));
    xasm.store();
    xasm.push(Value::function(
        |xasm: &mut Machine| {
            xasm.push(Value::string("a"));
            xasm.store();
            xasm.push(Value::function(
                |xasm: &mut Machine| {
                    xasm.push(Value::string("b"));
                    xasm.store();
                    xasm.push(Value::string("b"));
                    xasm.load();
                },
                &xasm,
            ));
        },
        &xasm,
    ));
    xasm.copy();
    xasm.push(Value::string("False"));
    xasm.store();
    xasm.push(Value::function(
        |xasm: &mut Machine| {
            xasm.push(Value::string("n"));
            xasm.store();
            xasm.push(Value::function(
                |xasm: &mut Machine| {
                    xasm.push(Value::string("f"));
                    xasm.store();
                    xasm.push(Value::function(
                        |xasm: &mut Machine| {
                            xasm.push(Value::string("x"));
                            xasm.store();
                            xasm.push(Value::function(
                                |xasm: &mut Machine| {
                                    xasm.push(Value::string("u"));
                                    xasm.store();
                                    xasm.push(Value::string("u"));
                                    xasm.load();
                                },
                                &xasm,
                            ));
                            xasm.push(Value::function(
                                |xasm: &mut Machine| {
                                    xasm.push(Value::string("u"));
                                    xasm.store();
                                    xasm.push(Value::string("x"));
                                    xasm.load();
                                },
                                &xasm,
                            ));
                            xasm.push(Value::function(
                                |xasm: &mut Machine| {
                                    xasm.push(Value::string("g"));
                                    xasm.store();
                                    xasm.push(Value::function(
                                        |xasm: &mut Machine| {
                                            xasm.push(Value::string("h"));
                                            xasm.store();
                                            xasm.push(Value::string("f"));
                                            xasm.load();
                                            xasm.push(Value::string("g"));
                                            xasm.load();
                                            xasm.call();
                                            xasm.push(Value::string("h"));
                                            xasm.load();
                                            xasm.call();
                                        },
                                        &xasm,
                                    ));
                                },
                                &xasm,
                            ));
                            xasm.push(Value::string("n"));
                            xasm.load();
                            xasm.call();
                            xasm.call();
                            xasm.call();
                        },
                        &xasm,
                    ));
                },
                &xasm,
            ));
        },
        &xasm,
    ));
    xasm.copy();
    xasm.push(Value::string("Pred"));
    xasm.store();
    xasm.push(Value::function(
        |xasm: &mut Machine| {
            xasm.push(Value::string("n"));
            xasm.store();
            xasm.push(Value::function(
                |xasm: &mut Machine| {
                    xasm.push(Value::string("f"));
                    xasm.store();
                    xasm.push(Value::function(
                        |xasm: &mut Machine| {
                            xasm.push(Value::string("x"));
                            xasm.store();
                            xasm.push(Value::string("x"));
                            xasm.load();
                            xasm.push(Value::string("f"));
                            xasm.load();
                            xasm.push(Value::string("n"));
                            xasm.load();
                            xasm.call();
                            xasm.call();
                            xasm.push(Value::string("f"));
                            xasm.load();
                            xasm.call();
                        },
                        &xasm,
                    ));
                },
                &xasm,
            ));
        },
        &xasm,
    ));
    xasm.copy();
    xasm.push(Value::string("Succ"));
    xasm.store();
    xasm.push(Value::string("False"));
    xasm.load();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.copy();
    xasm.push(Value::string("Once"));
    xasm.store();
    xasm.push(Value::function(
        |xasm: &mut Machine| {
            xasm.push(Value::string("n"));
            xasm.store();
            xasm.push(Value::string("n"));
            xasm.load();
        },
        &xasm,
    ));
    xasm.copy();
    xasm.push(Value::string("testing"));
    xasm.store();
    xasm.push(Value::function(
        |xasm: &mut Machine| {
            xasm.push(Value::string("x"));
            xasm.store();
            xasm.push(Value::string("y"));
            xasm.store();
            xasm.push(Value::string("x"));
            xasm.load();
            xasm.push(Value::string("y"));
            xasm.load();
        },
        &xasm,
    ));
    xasm.copy();
    xasm.push(Value::string("add"));
    xasm.store();
    xasm.push(Value::string("Once"));
    xasm.load();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Pred"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Pred"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Pred"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Pred"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("to_num"));
    xasm.load();
    xasm.call();

    println!("{}", xasm.pop().unwrap());
    xasm.push(Value::string("Once"));
    xasm.load();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Pred"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Pred"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Pred"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Pred"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("to_num"));
    xasm.load();
    xasm.call();

    println!("{}", xasm.pop().unwrap());
    xasm.push(Value::string("Once"));
    xasm.load();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Pred"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Pred"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Pred"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Pred"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("to_num"));
    xasm.load();
    xasm.call();

    println!("{}", xasm.pop().unwrap());
    xasm.push(Value::string("Once"));
    xasm.load();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Pred"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Pred"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Pred"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Pred"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("to_num"));
    xasm.load();
    xasm.call();

    println!("{}", xasm.pop().unwrap());
    xasm.push(Value::string("Once"));
    xasm.load();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Pred"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Pred"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Pred"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Pred"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("to_num"));
    xasm.load();
    xasm.call();

    println!("{}", xasm.pop().unwrap());
    xasm.push(Value::string("Once"));
    xasm.load();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Pred"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Succ"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Pred"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Pred"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("Pred"));
    xasm.load();
    xasm.call();
    xasm.push(Value::string("to_num"));
    xasm.load();
    xasm.call();

    println!("{}", xasm.pop().unwrap());
}
