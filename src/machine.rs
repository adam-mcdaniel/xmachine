use crate::{Ref, Value};

// We need BTreeMap to implement the 'Heap' (registers)
use alloc::collections::BTreeMap;
// For ToString generics
use alloc::string::{String, ToString};
// We need Vec for the dynamically allocated stack
use alloc::vec::Vec;
// For implementing Display and Debug
use core::fmt::{Display, Error, Formatter};

#[derive(Default, Clone, PartialEq)]
pub struct Machine {
    /// A dynamically allocated stack to push and pop values onto and off of
    pub stack: Vec<Ref<Value>>,
    /// The place to store named values (variables)
    pub registers: BTreeMap<String, Ref<Value>>,
}

impl Machine {
    /// Return a new instance of a Machine with empty stack and registers
    pub fn new() -> Self {
        Machine {
            stack: Vec::new(),
            registers: BTreeMap::new(),
        }
    }

    /// Push an item onto the stack
    pub fn push(&mut self, value: Ref<Value>) {
        self.stack.push(value);
    }

    /// Pop an item off of the stack, and return it
    pub fn pop(&mut self) -> Option<Ref<Value>> {
        self.stack.pop()
    }

    /// 1) Pop off a REFERENCE value from the stack
    /// 2) Push a copy the object and remove the reference
    pub fn copy(&mut self) {
        let reference = self.pop();
        if let Some(v) = reference {
            self.push(Ref::new((*v).clone()));
        }
    }

    /// 1) Pop off a REFERENCE value from the stack
    /// 2) Pop off a VALUE value from the stack
    /// 3) Assign the value of VALUE to the memory location of REFERENCE
    ///
    /// This can be used to assign to an indexed value from a list or table
    pub fn assign(&mut self) {
        let reference = self.pop();
        let value = self.pop();

        if let (Some(r), Some(v)) = (reference, value) {
            // We cant 'safely' modify a shared reference to a value,
            // so we need to convert to a mutable pointer in an unsafe block
            unsafe {
                // Take the Ref and convert it to a mutable pointer
                let ptr = Ref::into_raw(r) as *mut Value;
                // Assign to the contents of the mutable pointer
                *ptr = (*v).clone();
                // Re-wrap the pointer in a Ref value to properly manage it again
                Ref::from_raw(ptr as *const Value);
            }
        }
    }

    /// 1) Pop off the INDEX value from the stack
    /// 2) Pop off a TABLE value from the stack
    /// 3) Push the TABLE[INDEX] reference onto the stack
    pub fn index(&mut self) {
        let index = self.pop();
        let table = self.pop();

        if let (Some(t), Some(i)) = (table, index) {
            let result;
            // We cant 'safely' modify a shared reference to a value,
            // so we need to convert to a mutable pointer in an unsafe block
            unsafe {
                // Take the Ref and convert it to a mutable pointer
                let ptr = Ref::into_raw(t) as *mut Value;
                // Assign to the contents of the mutable pointer
                result = (*ptr).index(i);
                // Re-wrap the pointer in a Ref value to properly manage it again
                Ref::from_raw(ptr as *const Value);
            }
            self.push(result);
        }
    }

    /// 1) Pop off the INDEX value from the stack
    /// 2) Pop off a TABLE value from the stack
    /// 3) Push the TABLE onto the stack
    /// 4) Call the value at TABLE[INDEX] as a function
    pub fn method_call(&mut self) {
        let index = self.pop();
        let table = self.pop();

        if let (Some(t), Some(i)) = (table, index) {
            // This is the `self` value to be passed to the function
            // The `self` value cannot be directly assigned to,
            // HOWEVER, its members / attributes can be assigned to
            self.push(Ref::clone(&t));
            self.push(t);
            self.push(i);
            self.index();
            self.call();
        }
    }

    /// 1) Pop off function from the stack
    /// 2) Call it with this Machine instance
    pub fn call(&mut self) {
        let function = self.pop();

        if let Some(f) = function {
            f.call(self);
        }
    }

    /// 1) Pop off a CONDITION function from the stack
    /// 2) Pop off a BODY function from the stack
    /// 3) Call the CONDITION function with the context of this instance
    /// 4) If the return value is true, run the BODY function with the context of this instance
    /// 5) Goto step 3
    pub fn while_loop(&mut self) {
        let condition = self.pop();
        let body = self.pop();
        if let (Some(c), Some(b)) = (condition, body) {
            // This will take the top item of the stack and convert it to a bool
            let get_condition = |machine: &mut Machine| -> bool {
                match machine.pop() {
                    Some(v) => (*v).clone().into(),
                    None => false,
                }
            };

            // First, get the condition
            c.call_global(self);
            while get_condition(self) {
                // If the condition is true, run the body of the while loop
                b.call_global(self);
                // Push the condition again to test on the next iteration
                c.call_global(self);
            }
        }
    }

    /// 1) Pop off a KEY value from the stack
    /// 2) Pop off a VALUE value from the stack
    /// 3) Assign the value of VALUE to the register named KEY
    pub fn store(&mut self) {
        // The register to assign to
        let key = self.pop();
        // The value to assign to it
        let value = self.pop();

        if let (Some(k), Some(v)) = (key, value) {
            // registers[key] = value
            self.registers.insert((*k).to_string(), v);
        }
    }

    /// 1) Pop off a KEY value from the stack
    /// 2) Push the value in the register named KEY to the stack
    pub fn load(&mut self) {
        let key_option = self.pop();
        if let Some(k) = key_option {
            // Push a cloned reference to the stack
            let key = &(*k).to_string();

            // The reason we don't do an if-let expression here is the fact
            // that we can't borrow self as both mutable and immutable at once
            if self.registers.contains_key(key) {
                self.push(Ref::clone(self.registers.get(key).unwrap()));
            } else {
                // If the register doesn't exist, push an Error instead
                self.push(Value::error("No register named ".to_string() + key));
            }
        }
    }
}

/// How to print Machine / convert Machine to string
/// This is for debugging code and seeing the current instance of the machine
impl Display for Machine {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(
            f,
            "Machine {{\n\tstack: {:?}\n\theap:  {:?}\n}}",
            self.stack, self.registers
        )
    }
}
