extern crate xmachine;
use xmachine::{Machine, Value};

#[cfg(test)]
mod closure_tests {
    use super::*;

    #[test]
    fn clone_foreign_state() {
        let n = 0;

        let mut m = Machine::new();
        m.push(Value::function(
            move |_: &mut Machine| {
                let mut n = n.clone();
                n += 1;
                assert_eq!(n, 1)
            },
            &m,
        ));
        m.call();

        assert_eq!(n, 0);
    }
}
