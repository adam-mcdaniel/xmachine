extern crate xmachine;
use xmachine::{Machine, Value};

#[cfg(test)]
mod object {
    use super::*;

    fn dict(xasm: &mut Machine) {
        xasm.push(Value::tree());
    }

    #[test]
    fn object() {
        let mut xasm = Machine::new();

        xasm.push(Value::function(dict, &xasm));
        xasm.copy();
        xasm.push(Value::string("new"));
        xasm.store();
        xasm.push(Value::function(
            |xasm: &mut Machine| {
                xasm.push(Value::string("new"));
                xasm.load();
                xasm.call();
                xasm.copy();
                xasm.push(Value::string("self"));
                xasm.store();
                xasm.push(Value::function(
                    |xasm: &mut Machine| {
                        xasm.push(Value::string("self"));
                        xasm.store();
                        xasm.push(Value::string("value"));
                        xasm.store();
                        xasm.push(Value::string("sign"));
                        xasm.store();
                        xasm.push(Value::string("value"));
                        xasm.load();
                        xasm.copy();
                        xasm.push(Value::string("self"));
                        xasm.load();
                        xasm.push(Value::string("value"));
                        xasm.index();
                        xasm.assign();
                        xasm.push(Value::string("sign"));
                        xasm.load();
                        xasm.copy();
                        xasm.push(Value::string("self"));
                        xasm.load();
                        xasm.push(Value::string("sign"));
                        xasm.index();
                        xasm.assign();
                        xasm.push(Value::string("self"));
                        xasm.load();
                    },
                    &xasm,
                ));
                xasm.copy();
                xasm.push(Value::string("self"));
                xasm.load();
                xasm.push(Value::string("constructor"));
                xasm.index();
                xasm.assign();
                xasm.push(Value::function(
                    |xasm: &mut Machine| {
                        xasm.push(Value::string("self"));
                        xasm.store();
                        xasm.push(Value::string("value"));
                        xasm.store();
                        xasm.push(Value::string("value"));
                        xasm.load();
                        xasm.copy();
                        xasm.push(Value::string("self"));
                        xasm.load();
                        xasm.push(Value::string("value"));
                        xasm.index();
                        xasm.assign();
                    },
                    &xasm,
                ));
                xasm.copy();
                xasm.push(Value::string("self"));
                xasm.load();
                xasm.push(Value::string("set_value"));
                xasm.index();
                xasm.assign();
                xasm.push(Value::string("self"));
                xasm.load();
            },
            &xasm,
        ));
        xasm.copy();
        xasm.push(Value::string("Number"));
        xasm.store();
        xasm.push(Value::number(0));
        xasm.push(Value::number(5));
        xasm.push(Value::string("Number"));
        xasm.load();
        xasm.call();
        xasm.push(Value::string("constructor"));
        xasm.method_call();
        xasm.push(Value::string("value"));
        xasm.index();

        assert_eq!(i32::from(xasm.get_arg()), 5);
    }
}
