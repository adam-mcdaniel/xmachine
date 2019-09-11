extern crate xmachine;
use xmachine::{Machine, Value};

#[cfg(test)]
mod church {
    use super::*;

    /// Tests that functions can return values to the stack
    #[test]
    fn church() {
        let mut m = Machine::new();

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

        m.push(Value::function(
            |m: &mut Machine| {
                m.push(Value::string("a"));
                m.store();
                m.push(Value::function(
                    |m: &mut Machine| {
                        m.push(Value::string("b"));
                        m.store();
                        m.push(Value::string("b"));
                        m.load();
                    },
                    &m,
                ));
            },
            &m,
        ));
        m.push(Value::string("False"));
        m.store();

        m.push(Value::number(0));
        m.push(Value::number(1));
        m.push(Value::string("True"));
        m.load();
        m.call();
        m.call();

        assert_eq!(f64::from(m.get_arg()), 1.0);

        m.push(Value::number(0));
        m.push(Value::number(1));
        m.push(Value::string("False"));
        m.load();
        m.call();
        m.call();

        assert_eq!(f64::from(m.get_arg()), 0.0);

        m.push(Value::number(1));
        m.push(Value::number(0));
        m.push(Value::string("True"));
        m.load();
        m.call();
        m.call();

        assert_eq!(i32::from(m.get_arg()), 0);
    }
}
