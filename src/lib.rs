#![no_std]

extern crate alloc;
use alloc::vec::Vec;
// use alloc::rc::Rc;
use alloc::sync::Arc;
use alloc::string::{String, ToString};
use alloc::collections::BTreeMap;
use core::fmt::{Display, Formatter, Error};

pub type Ref<T> = Arc<T>;


#[derive(Clone)]
pub enum Value {
    String(String),
    Number(f64),
    List(Vec<Self>),
    Tree(BTreeMap<String, Self>),
    Function(fn(&mut Machine) -> ()),
    Error(String),
    None
}


impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Self::String(s) => write!(f, "{}", s),
            Self::Number(n) => write!(f, "{}", n),
            Self::List(_) => write!(f, "<List>"),
            Self::Tree(_) => write!(f, "<Tree>"),
            Self::Function(_) =>  write!(f, "<Fn>"),
            Self::Error(s) =>  write!(f, "<Exception: '{}'>", s),
            Self::None =>  write!(f, "None"),
        }
    }
}


#[derive(Default)]
pub struct Machine {
    stack: Vec<Ref<Value>>,
    registers: BTreeMap<String, Ref<Value>>
}

impl Machine {
    pub fn new() -> Self {
        Machine {
            stack: Vec::new(),
            registers: BTreeMap::new()
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
    /// 2) Pop off a VALUE value from the stack
    /// 3) Assign the value of VALUE to the memory location of REFERENCE
    /// 
    /// This can be used to assign to an indexed value from a list or table
    pub fn assign(&mut self) {
        let reference = self.pop();
        let value = self.pop();

        match (reference, value) {
            (Some(r), Some(v)) => {
                unsafe {
                    let ptr = Ref::into_raw(r) as *mut Value;
                    *ptr = (*v).clone();
                    Ref::from_raw(ptr as *const Value);
                }
            },
            (Some(r), _) => {
                self.push(
                    Ref::new(Value::Error(
                        String::from("Could not assign to ") + &r.to_string()
                    ))
                );
            },
            (_, _) => {}
        }

    }

    /// 1) Pop off a KEY value from the stack
    /// 2) Pop off a VALUE value from the stack
    /// 3) Assign the value of VALUE to the register named KEY
    pub fn store(&mut self) {
        let key = self.pop();
        let value = self.pop();

        match (key, value) {
            (None, None) => {},
            (Some(k), Some(v)) => {
                // registers[key] = value
                self.registers.insert((*k).to_string(), v);
            },
            (_, _) => {}
        }
    }


    /// 1) Pop off a KEY value from the stack
    /// 2) Push the value in the register named KEY to the stack
    pub fn load(&mut self) {
        let key = self.pop();
        match key {
            Some(k) => {
                // Push a cloned reference to the stack
                let key = &(*k).to_string();
                if self.registers.contains_key(key) {
                    self.push(
                        Ref::clone(self.registers.get(key).unwrap())
                    );
                } else {
                    self.push(
                        Ref::new(Value::Error(
                            String::from("No register named ") + key
                        ))
                    );
                }

            },
            None => {},
        }
    }
}


impl Display for Machine {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Machine:\n  [")?;
        for item in &self.stack {
            write!(f, " {}", item.clone())?;
        }
        write!(f, " ]")?;
        Ok(())
    }
}
