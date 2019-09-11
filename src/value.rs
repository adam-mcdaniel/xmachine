use crate::{Function, Machine, Ref};
use core::ops::{Add, Div, Mul, Not, Rem, Sub};

// We need BTreeMap to implement the Tree type
use alloc::collections::BTreeMap;
// For ToString generics
use alloc::string::{String, ToString};
// We need Vec for dynamically allocated lists
use alloc::vec::Vec;
// For implementing Display and Debug
use core::fmt::{Debug, Display, Error, Formatter};

#[derive(Clone, PartialEq, PartialOrd)]
pub enum Value {
    String(String),
    Number(f64),
    List(Vec<Ref<Self>>),
    Tree(BTreeMap<String, Ref<Self>>),
    Function(Function<Machine, (), Machine>),
    Error(String),
    None,
}

impl Value {
    /// Creates a new reference to a Number
    pub fn number<N: Into<f64>>(n: N) -> Ref<Self> {
        Ref::new(Self::Number(n.into()))
    }

    /// Creates a new reference to a String
    pub fn string<S: ToString>(s: S) -> Ref<Self> {
        Ref::new(Self::String(s.to_string()))
    }

    /// Creates a new reference to an empty List
    pub fn list() -> Ref<Self> {
        Ref::new(Self::List(Vec::new()))
    }

    /// Creates a new reference to an empty Tree
    pub fn tree() -> Ref<Self> {
        Ref::new(Self::Tree(BTreeMap::new()))
    }

    /// Creates a reference to a Function with a captured context, basically a Closure
    pub fn function(f: impl 'static + Fn(&mut Machine) -> (), context: &Machine) -> Ref<Self> {
        Ref::new(Self::Function(Function::new(
            f,
            context.clone().duplicate(),
        )))
    }

    /// Creates a reference to an Error value
    pub fn error<S: ToString>(s: S) -> Ref<Self> {
        Ref::new(Self::Error(s.to_string()))
    }

    /// Creates a reference to an None value
    pub fn none() -> Ref<Self> {
        Ref::new(Self::None)
    }

    /// Copies the contents of this value
    pub fn copy(&self) -> Ref<Self> {
        // In the future, if memory leaks become a problem,
        // we could try replacing the item clone with an
        // item copy.
        // This would recursively call copy to ensure no
        // Refs are the same. It might be that we never
        // need to change this, though.
        match self {
            Self::List(l) => {
                let mut list = vec![];
                for item in l {
                    list.push(Ref::new((**item).clone()));
                }
                Ref::new(Self::List(list))
            }
            Self::Tree(l) => {
                let mut map = BTreeMap::new();
                for (name, item) in l {
                    map.insert(name.clone(), Ref::new((**item).clone()));
                }
                Ref::new(Self::Tree(map))
            }
            _ => Ref::new(self.clone()),
        }
    }

    /// Call this function in the context of the Machine
    /// captured when this instance of the function was created
    pub fn call(&self, machine: &mut Machine) {
        if let Self::Function(f) = self {
            // Get the captured machine back from the function
            let mut temp_machine = f.get_context().clone();
            // Give it the current machine's stack
            temp_machine.stack = machine.stack.clone();
            // Call the function with the new machine
            f.call(&mut temp_machine);
            // Give back the modified stack
            machine.stack = temp_machine.stack;
        }
    }

    /// Call this function in the context of the current machine,
    /// meaning, execute the instructions of this function as if
    /// they were not in a function.
    pub fn call_global(&self, machine: &mut Machine) {
        if let Self::Function(f) = self {
            // Call the function with the given machine
            f.call(machine);
        }
    }

    pub fn is_err(&self) -> bool {
        match self {
            Self::Error(_) => true,
            _ => false,
        }
    }

    /// Return a reference to a value contained within a collection
    pub fn index<S: ToString>(&mut self, s: S) -> Ref<Self> {
        let key = s.to_string();
        match self {
            Self::String(s) => {
                match key.parse::<usize>() {
                    Ok(n) => {
                        if s.len() > n {
                            Value::string(s.chars().nth(n).unwrap().to_string())
                        } else {
                            Self::error("String index out of bounds")
                        }
                    }
                    Err(_) => Self::error("Can't index string with non-integer"),
                }
            }
            Self::Tree(t) => {
                // If the current tree does not have a
                // key with this name, create one
                if !t.contains_key(&key) {
                    t.insert(key.clone(), Self::none());
                }

                // Return a reference to this object in the table
                Ref::clone(t.get(&key).unwrap())
            }
            Self::List(l) => {
                // Convert to usize to index this value as a list
                match key.parse::<usize>() {
                    Ok(n) => {
                        // If the requested index is too high, allocate space for it and continue
                        if n >= l.len() {
                            // Reserve space for new size
                            // This is good because it minimizes the
                            // number of numerous, small allocations.
                            l.reserve(n - l.len() + 1);

                            // Fill the space with None
                            for _ in l.len()..=n {
                                l.push(Self::none());
                            }
                        }

                        // Return reference to the requested item in the list
                        Ref::clone(&l[n])
                    }
                    // Could not convert key to usize
                    Err(_) => Self::error("Can't index list with non-integer"),
                }
            }
            // Tried to index something other than list or tree
            _ => Self::error("Can't index non-list or non-tree"),
        }
    }
}

/// This implementation is a hack for implementing Display for Value
impl Debug for Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self)
    }
}

/// How to display value
impl Display for Value {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Self::String(s) => write!(f, "{}", s),
            Self::Number(n) => write!(f, "{}", n),
            Self::List(l) => write!(f, "{:?}", l), // Requires the dummy debug implementation above
            Self::Tree(t) => write!(f, "{:?}", t), // Requires the dummy debug implementation above
            Self::Function(func) => write!(f, "{}", func),
            Self::Error(s) => write!(f, "<Exception: '{}'>", s),
            Self::None => write!(f, "None"),
        }
    }
}

/// ############################################################
/// The following traits are for implementing foreign functions!
/// ############################################################

/// Convert Value into a bool
impl From<Value> for bool {
    fn from(v: Value) -> Self {
        match v {
            Value::String(s) => s != "", // self != ""
            Value::Number(n) => (if n < 0.0 { -n } else { n }) > 0.000_000_000_1, // self is non-zero
            Value::List(l) => !l.is_empty(),                                      // self is not []
            Value::Tree(t) => !t.is_empty(),                                      // self is not {}
            Value::Function(_) => true, // functions are true values
            Value::Error(_) => false,   // errors are false values
            Value::None => false,       // nones are false values
        }
    }
}

/// Get a function from the value
impl From<Value> for String {
    fn from(v: Value) -> Self {
        match v {
            Value::String(s) => s,
            Value::Error(e) => e,
            _ => String::from(""),
        }
    }
}

/// Get a function from the value
impl From<Value> for Function<Machine, (), Machine> {
    fn from(v: Value) -> Self {
        match v {
            Value::Function(f) => f,
            _ => Function::new(|_: &mut Machine| {}, Machine::new()),
        }
    }
}

/// Convert Value to unwrapped List
impl From<Value> for Vec<Ref<Value>> {
    fn from(v: Value) -> Self {
        match v {
            Value::List(l) => l,
            _ => Vec::new(),
        }
    }
}

/// Convert Value to unwrapped Tree
impl From<Value> for BTreeMap<String, Ref<Value>> {
    fn from(v: Value) -> Self {
        match v {
            Value::Tree(t) => t,
            _ => BTreeMap::new(),
        }
    }
}

/// Convert to floating point value
impl From<Value> for f64 {
    fn from(v: Value) -> Self {
        match v {
            Value::Number(n) => n,
            _ => 0.0,
        }
    }
}

/// Convert to integer value
impl From<Value> for i32 {
    fn from(v: Value) -> Self {
        match v {
            Value::Number(n) => n as i32,
            _ => 0,
        }
    }
}

/// Make Value from String
impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

/// Make Value from &str
impl From<&str> for Value {
    fn from(s: &str) -> Self {
        Value::String(s.to_string())
    }
}

/// Make Value from bool
impl From<bool> for Value {
    fn from(n: bool) -> Self {
        Value::Number(f64::from(n as i32))
    }
}

/// Make Value from Number
impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Value::Number(n)
    }
}

/// Make Value from Number
impl From<i32> for Value {
    fn from(n: i32) -> Self {
        Value::Number(f64::from(n))
    }
}

/// Make Value from List
impl From<Vec<Ref<Value>>> for Value {
    fn from(l: Vec<Ref<Value>>) -> Self {
        Value::List(l)
    }
}

/// Make Value from Tree
impl From<BTreeMap<String, Ref<Value>>> for Value {
    fn from(t: BTreeMap<String, Ref<Value>>) -> Self {
        Value::Tree(t)
    }
}

/// Make Value from Function
impl From<Function<Machine, (), Machine>> for Value {
    fn from(f: Function<Machine, (), Machine>) -> Self {
        Value::Function(f)
    }
}

/// ##############################################################
/// The following traits are for implementing operators and logic!
/// ##############################################################

/// Add two values
impl Add<Value> for Value {
    type Output = Value;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            // Concat two strings
            (Self::String(s1), Self::String(s2)) => Self::String(s1 + &s2),
            // Add two numbers
            (Self::Number(m), Self::Number(n)) => Self::Number(m + n),
            // Concat two lists
            (Self::List(mut l1), Self::List(l2)) => {
                l1.extend(l2);
                Self::List(l1)
            }
            // Otherwise, return exception
            (a, b) => Self::Error(format!("Could not add {} and {}", a, b)),
        }
    }
}

/// Subtract two values
impl Sub<Value> for Value {
    type Output = Value;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            // Subtract two numbers
            (Self::Number(m), Self::Number(n)) => Self::Number(m - n),
            // Otherwise, return exception
            (a, b) => Self::Error(format!("Could not subtract {} and {}", a, b)),
        }
    }
}

/// Multiply two values
impl Mul<Value> for Value {
    type Output = Value;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            // Repeat a string
            (Self::String(s1), Self::Number(n)) => Self::String(s1.repeat(n as usize)),
            // Multiply two numbers
            (Self::Number(m), Self::Number(n)) => Self::Number(m * n),
            // Otherwise, return exception
            (a, b) => Self::Error(format!("Could not multiply {} and {}", a, b)),
        }
    }
}

/// Divide two values
impl Div<Value> for Value {
    type Output = Value;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            // Divide two numbers
            (Self::Number(m), Self::Number(n)) => Self::Number(m / n),
            // Otherwise, return exception
            (a, b) => Self::Error(format!("Could not divide {} and {}", a, b)),
        }
    }
}

/// Remainder of two values
impl Rem<Value> for Value {
    type Output = Value;
    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            // Remainder of two numbers
            (Self::Number(m), Self::Number(n)) => Self::Number(m % n),
            // Otherwise, return exception
            (a, b) => Self::Error(format!("Could not find the remainder of {} and {}", a, b)),
        }
    }
}

/// Negate value
impl Not for Value {
    type Output = Value;
    fn not(self) -> Self::Output {
        match self {
            // Negate number
            Self::Number(n) => match n as i32 {
                // If number is zero, return true
                0 => Self::Number(1.0),
                // If number is not zero, return false
                _ => Self::Number(0.0),
            },
            a => Self::Error(format!("Could not negate {}", a)),
        }
    }
}
