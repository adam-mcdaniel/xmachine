use crate::Ref;
use core::fmt::{Display, Error, Formatter};

use alloc::string::ToString;

/// Represents a function that takes a &mut I, returns O,
/// and contains a captured context C.
#[derive(Clone)]
pub struct Function<I, O, C> {
    /// Function pointer to call
    function_ptr: Ref<dyn Fn(&mut I) -> O>,
    /// The captured context of the function
    context: C,
}

/// Represents a function that takes a &mut I, returns O,
/// and contains a captured context C.
impl<I, O, C> Function<I, O, C> {
    /// Create a function from a function pointer and captured context
    /// We use a function pointer because a non-capturing lambda can
    /// decay into a function pointer, and because it's sized!
    pub fn new(function_ptr: impl 'static + Fn(&mut I) -> O, context: C) -> Self {
        Self {
            function_ptr: Ref::new(function_ptr),
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
        let ptr = Ref::into_raw(self.function_ptr.clone()) as *const u8;
        write!(f, "<fn at {}>", format!("{:?}", ptr)[..8].to_string())?;
        unsafe {
            Ref::from_raw(ptr);
        }
        Ok(())
    }
}

/// == operator for Function
/// This doesn't compare the function pointer,
/// but instead compares the contexts. If the contexts are the same,
/// it's almost guaranteed that these two functions are the same.
impl<I, O, C> PartialEq for Function<I, O, C> {
    fn eq(&self, rhs: &Self) -> bool {
        format!("{}", self) == format!("{}", rhs)
    }
}

/// Ord operators for Function
/// This doesn't compare the function pointer,
/// but instead compares the contexts.
impl<I, O, C: PartialOrd> PartialOrd for Function<I, O, C> {
    fn partial_cmp(&self, rhs: &Self) -> Option<core::cmp::Ordering> {
        self.context.partial_cmp(&rhs.context)
    }
}

impl<I, O, C> Default for Function<I, O, C>
where
    I: Default,
    O: Default,
    C: Default,
{
    fn default() -> Self {
        Self::new(|_: &mut I| Default::default(), Default::default())
    }
}
