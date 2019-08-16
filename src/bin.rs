extern crate xmachine;
use xmachine::{Machine, Value};

fn main() {
    let mut m = Machine::new();

    m.push(Value::list());
    m.push(Value::string("test"));
    m.store();
    m.push(Value::tree());
    m.push(Value::number(2));
    m.push(Value::string("test"));
    m.load();
    m.index();
    m.assign();

    m.push(Value::function(
        |m: &mut Machine| {
            m.push(Value::string("a"));
            m.store();
            m.push(Value::function(
                |m: &mut Machine| {
                    m.push(Value::string("b"));
                    m.store();
                    m.push(Value::string("a"));
                    m.load();
                },
                &m,
            ));
        },
        &m,
    ));
    m.push(Value::string("True"));
    m.store();

    m.push(Value::number(0));
    m.push(Value::number(1));
    m.push(Value::string("True"));
    m.load();
    m.call();
    m.call();

    m.push(Value::number(10));
    m.push(Value::string("counter"));
    m.store();

    m.push(Value::function(
        |m: &mut Machine| {
            m.push(Value::number(0));
            m.push(Value::string("counter"));
            m.store();
        },
        &m,
    ));
    m.push(Value::function(
        |m: &mut Machine| {
            m.push(Value::string("counter"));
            m.load();
        },
        &m,
    ));
    m.while_loop();

    println!("{}", m);
}
