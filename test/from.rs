extern crate xmachine;
use xmachine::{Ref, Value};

extern crate alloc;
use alloc::collections::BTreeMap;

#[cfg(test)]
mod from {
    use super::*;

    #[test]
    fn from_string() {
        assert_eq!(Value::string("test"), Ref::new(Value::from("test")));

        assert_eq!(String::from(Value::from("test")), String::from("test"));
    }

    #[test]
    fn from_tree() {
        assert_eq!(Value::tree(), Ref::new(Value::from(BTreeMap::new())));
    }

    #[test]
    fn from_list() {
        assert_eq!(Value::list(), Ref::new(Value::from(Vec::new())));
    }

    #[test]
    fn from_number() {
        assert_eq!(Value::number(5), Ref::new(Value::from(5)));
        assert_eq!(Value::number(5.9), Ref::new(Value::from(5.9)));
    }

    #[test]
    fn from_bool() {
        assert_eq!(bool::from(Value::Number(1.0)), true);
        assert_eq!(bool::from(Value::Number(5.1)), true);
        assert_eq!(bool::from(Value::Number(0.0)), false);
        assert_eq!(bool::from(Value::Number(-5.6)), true);
        assert_eq!(bool::from((*Value::tree()).clone()), false);
        assert_eq!(bool::from((*Value::list()).clone()), false);
        assert_eq!(bool::from(Value::String(String::from("test"))), true);
        assert_eq!(bool::from(Value::String(String::from(""))), false);
    }
}
