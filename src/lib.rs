#![no_std]
#[macro_use]
extern crate alloc;
use alloc::sync::Arc;

pub type Ref<T> = Arc<T>;

mod value;
pub use value::Value;

mod machine;
pub use machine::Machine;

mod function;
use function::Function;
