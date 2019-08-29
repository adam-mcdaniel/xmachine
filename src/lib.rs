#![no_std]
#[macro_use]
extern crate alloc;
use alloc::rc::Rc;

pub type Ref<T> = Rc<T>;

mod value;
pub use value::Value;

mod machine;
pub use machine::Machine;

mod function;
use function::Function;
