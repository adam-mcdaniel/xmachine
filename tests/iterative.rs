extern crate xmachine;
use xmachine::{Machine, Value};

#[cfg(test)]
mod iterative {
    use super::*;

    fn sub(m: &mut Machine) {
        let n1 = m.get_arg::<f64>();
        let n2 = m.get_arg::<f64>();
        m.return_value(n1 - n2);
    }

    #[test]
    fn while_loop() {
        let mut m = Machine::new();
        m.push(Value::function(sub, &m));
        m.push(Value::string("sub"));
        m.store();

        m.push(Value::number(5));
        m.push(Value::string("test"));
        m.store();
        m.push(Value::function(
            |m: &mut Machine| {
                m.push(Value::string("test"));
                m.load();
                m.push(Value::number(1));
                m.push(Value::string("test"));
                m.load();
                m.push(Value::string("sub"));
                m.load();
                m.call();
                m.push(Value::string("test"));
                m.store();
            },
            &m,
        ));
        m.push(Value::function(
            |m: &mut Machine| {
                m.push(Value::string("test"));
                m.load();
            },
            &m,
        ));
        m.while_loop();

        assert_eq!(
            m.stack,
            vec![
                Value::number(5),
                Value::number(4),
                Value::number(3),
                Value::number(2),
                Value::number(1),
            ]
        )
    }
}
