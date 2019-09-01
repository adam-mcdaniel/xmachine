extern crate xmachine;
use xmachine::{Machine, Value};

#[cfg(test)]
mod function_tests {
    use super::*;

    /// Tests that functions can return values to the stack
    #[test]
    fn function_return() {
        // Test single return value
        let mut m = Machine::new();
        m.push(Value::function(
            |m: &mut Machine| {
                m.push(Value::number(5));
            },
            &m,
        ));
        m.call();

        assert_eq!(m.stack, vec![Value::number(5)]);

        // Test multiple return values
        let mut m = Machine::new();
        m.push(Value::function(
            |m: &mut Machine| {
                m.push(Value::number(5));
                m.push(Value::string("whoa"));
                m.push(Value::tree());
            },
            &m,
        ));
        m.call();

        assert_eq!(
            m.stack,
            vec![Value::number(5), Value::string("whoa"), Value::tree()]
        );
    }

    /// This function tests the ability to pass a copy to a
    /// function and mutate it while not changing the original
    #[test]
    fn function_copy() {
        let mut m = Machine::new();

        // Store `yo yo yo` in `test`
        m.push(Value::string("yo yo yo"));
        m.push(Value::string("test"));
        m.store();
        // Store call function with argument 5
        m.push(Value::number(5));
        m.push(Value::function(
            |m: &mut Machine| {
                // Load `test` from global scope
                m.push(Value::string("test"));
                m.load();
                // Copy item on stack to remove reference
                m.copy();
                // Assign to the COPY's location in memory!
                m.assign();
            },
            &m,
        ));
        m.call();

        assert_eq!(
            m.registers[&String::from("test")],
            Value::string("yo yo yo")
        );
    }
}
