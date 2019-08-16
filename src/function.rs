use core::fmt::{Display, Error, Formatter};

/// Represents a function that takes a &mut I, returns O,
/// and contains a captured context C.
#[derive(Clone)]
pub struct Function<I, O, C> {
    /// Function pointer to call
    function_ptr: fn(&mut I) -> O,
    /// The captured context of the function
    context: C,
}

/// Represents a function that takes a &mut I, returns O,
/// and contains a captured context C.
impl<I, O, C> Function<I, O, C> {
    /// Create a function from a function pointer and captured context
    /// We use a function pointer because a non-capturing lambda can
    /// decay into a function pointer, and because it's sized!
    pub fn new(function_ptr: fn(&mut I) -> O, context: C) -> Self {
        Self {
            function_ptr,
            context,
        }
    }

    /// Return the captured context of the Function
    pub fn get_context(&self) -> &C {
        &self.context
    }

    /// Call this function with an input and return the output
    pub fn call(&self, input: &mut I) -> O {
        (self.function_ptr)(input)
    }
}

impl<I, O, C> Display for Function<I, O, C> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "<function at {:?}>", self.function_ptr as *const u8)
    }
}

/// Not really useful ATM
/// Functions with contexts that implement Copy can be Copied!
impl<I: Clone, O: Clone, C: Copy> Copy for Function<I, O, C> {}

/// == operator for Function
/// This doesn't compare the function pointer,
/// but instead compares the contexts. If the contexts are the same,
/// it's almost guaranteed that these two functions are the same.
impl<I, O, C: PartialEq> PartialEq for Function<I, O, C> {
    fn eq(&self, rhs: &Self) -> bool {
        self.context == rhs.context
    }
}
