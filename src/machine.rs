use crate::{Ref, Value};

// We need BTreeMap to implement the 'Heap' (registers)
use alloc::collections::BTreeMap;
// For ToString generics
use alloc::string::{String, ToString};
// We need Vec for the dynamically allocated stack
use alloc::vec::Vec;
// For implementing Display and Debug
use core::fmt::{Display, Error, Formatter};

#[derive(Default, Clone, PartialEq, PartialOrd)]
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

    /// ####################################################
    /// The following functions are meant to be used to
    /// interface and interact with the virtual machine
    /// ####################################################

    /// FOR FOREIGN FUNCTIONS
    /// This gets an argument from the call to this foreign
    /// function by popping a value off the stack, and removing
    /// the reference
    pub fn get_arg(&mut self) -> Value {
        (*self.pop()).clone()
    }

    /// FOR FOREIGN FUNCTIONS
    /// This pushes a return value onto the stack
    pub fn return_value(&mut self, value: Value) {
        self.push(Ref::new(value))
    }

    /// ####################################################
    /// The following functions represent instructions that
    /// are natively supported by the virtual machine. These
    /// are not meant to be used by foreign functions, but
    /// they CAN be used without worry.
    /// ####################################################

    /// This function duplicates the current machine. This is
    /// VERY IMPORTANT. It iterates through the stack and copies
    /// each item into a new machine.
    ///
    /// This is ONLY used to create the context for functions.
    /// If we don't do this, the context Machine never goes out
    /// of scope, and never lets the Refs die. This causes a
    /// memory leak.
    ///
    /// The addition of this method fixes the memory leak.
    pub fn duplicate(self) -> Self {
        let mut new = Self::new();
        // Copy the stack for the new machine
        for item in self.stack {
            new.push((*item).clone().copy());
        }

        // Copy the registers for the new machine
        for (key, value) in self.registers {
            new.registers.insert(key.to_string(), value.copy());
        }

        // Return new machine
        new
    }

    /// Push an item onto the stack
    pub fn push(&mut self, value: Ref<Value>) {
        self.stack.push(value);
    }

    /// Pop an item off of the stack, and return it
    /// If the stack is empty, return an Error
    pub fn pop(&mut self) -> Ref<Value> {
        match self.stack.pop() {
            Some(v) => v,
            None => Value::error("Popped from empty stack, called function with too few arguments"),
        }
    }

    /// 1) Pop off a REFERENCE value from the stack
    /// 2) Push a copy the object and remove the reference
    pub fn copy(&mut self) {
        let value = self.pop();
        self.push(value.copy());
    }

    /// 1) Pop off a REFERENCE value from the stack
    /// 2) Pop off a VALUE value from the stack
    /// 3) Assign the value of VALUE to the memory location of REFERENCE
    ///
    /// This can be used to assign to an indexed value from a list or table
    pub fn assign(&mut self) {
        let reference = self.pop();
        let value = self.pop();

        // We cant 'safely' modify a shared reference to a value,
        // so we need to convert to a mutable pointer in an unsafe block
        unsafe {
            // Take the Ref and convert it to a mutable pointer
            let ptr = Ref::into_raw(reference) as *mut Value;
            // Assign to the contents of the mutable pointer
            *ptr = (*value).clone();
            // Re-wrap the pointer in a Ref value to properly manage it again
            Ref::from_raw(ptr as *const Value);
        }
    }

    /// 1) Pop off the INDEX value from the stack
    /// 2) Pop off a TABLE value from the stack
    /// 3) Push the TABLE[INDEX] reference onto the stack
    pub fn index(&mut self) {
        let index = self.pop();
        let table = self.pop();

        let result;
        // We cant 'safely' modify a shared reference to a value,
        // so we need to convert to a mutable pointer in an unsafe block
        unsafe {
            // Take the Ref and convert it to a mutable pointer
            let ptr = Ref::into_raw(table) as *mut Value;
            // Get the indexed value from the pointer to the table in memory
            result = (*ptr).index(index);
            // Re-wrap the pointer in a Ref value to properly manage it again
            Ref::from_raw(ptr as *const Value);
        }
        self.push(result);
    }

    /// 1) Pop off the INDEX value from the stack
    /// 2) Pop off a TABLE value from the stack
    /// 3) Push the TABLE onto the stack
    /// 4) Call the value at TABLE[INDEX] as a function
    pub fn method_call(&mut self) {
        let index = self.pop();
        let table = self.pop();

        // This is the `self` value to be passed to the function
        // The `self` value cannot be directly assigned to,
        // HOWEVER, its members / attributes can be assigned to
        self.push(Ref::clone(&table));
        self.push(table);
        self.push(index);
        self.index();
        self.call();
    }

    /// 1) Pop off function from the stack
    /// 2) Call it with this Machine instance
    pub fn call(&mut self) {
        let function = self.pop();
        function.call(self);
    }


    /// 1) Pop off a COUNTER identifier from the stack
    /// 2) Pop off an ELEMENT identifier from the stack
    /// 3) Pop off a LIST value from the stack
    /// 4) Pop off a BODY function from the stack
    /// 5) For COUNTER, ELEMENT in enumeration of LIST:
    /// 6)   Store LIST[COUNTER] in ELEMENT
    /// 7)   Call BODY with current instance
    /// 8)   Increment COUNTER
    pub fn for_loop(&mut self) {
        let counter_name = self.pop();
        let element_name = self.pop();
        let iterator = (*self.pop()).clone();
        let body = self.pop();

        for (index, item) in iterator.into_iter().enumerate() {
            self.registers.insert(element_name.to_string(), item);
            self.registers.insert(counter_name.to_string(), Value::number(index as f64));
            body.call_global(self);
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
        // This will take the top item of the stack and convert it to a bool
        let get_condition = |machine: &mut Machine| -> bool { (*machine.pop()).clone().into() };

        // First, get the condition
        condition.call_global(self);
        while get_condition(self) {
            // If the condition is true, run the body of the while loop
            body.call_global(self);
            // Push the condition again to test on the next iteration
            condition.call_global(self);
        }
    }

    /// 1) Pop off a CONDITION function from the stack
    /// 2) Pop off a THEN function from the stack
    /// 3) Pop off an ELSE function from the stack
    /// 4) Call the CONDITION function with the context of this instance
    /// 5) If the return value is true, run the THEN function with the context of this instance
    /// 6) If the return value is false, run the ELSE function with the context of this instance
    pub fn if_then_else(&mut self) {
        let condition = self.pop();
        let then_fn = self.pop();
        let else_fn = self.pop();

        // This will take the top item of the stack and convert it to a bool
        let get_condition = |machine: &mut Machine| -> bool { (*machine.pop()).clone().into() };

        // First, get the condition
        condition.call_global(self);
        if get_condition(self) {
            // If the condition is true, run the body of the while loop
            then_fn.call_global(self);
        } else {
            // Push the condition again to test on the next iteration
            else_fn.call_global(self);
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

        // registers[key] = value
        self.registers.insert(key.to_string(), value);
    }

    /// 1) Pop off a KEY value from the stack
    /// 2) Push the value in the register named KEY to the stack
    pub fn load(&mut self) {
        let key = &self.pop().to_string();

        // The reason we don't do an if-let expression here is the fact
        // that we can't borrow self as both mutable and immutable at once
        if self.registers.contains_key(key) {
            self.push(Ref::clone(self.registers.get(key).unwrap()));
        } else {
            self.push(Value::error(format!("No register named {}", key)));
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
