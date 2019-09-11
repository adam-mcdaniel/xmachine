extern crate xmachine;
use xmachine::{Machine, Ref, Value};

extern crate alloc;
use alloc::collections::BTreeMap;

#[cfg(test)]
mod value {
    use super::*;

    #[test]
    fn call() {
        let mut m = Machine::new();

        let f = Value::function(
            |m: &mut Machine| {
                m.return_value(Value::from(5));
            },
            &m,
        );

        f.call(&mut m);
        assert_eq!(m.stack, vec![Value::number(5)]);
    }

    #[test]
    fn display() {
        assert_eq!(String::from("test"), format!("{}", Value::string("test")))
    }

    #[test]
    fn index() {
        let mut map: BTreeMap<String, Ref<Value>> = BTreeMap::new();

        map.insert(String::from("test"), Value::number(5));

        assert_eq!(Value::from(map).index("test"), Value::number(5));
    }
}
