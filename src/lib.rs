#![no_std]

extern crate alloc;
use alloc::sync::Arc;

pub type Ref<T> = Arc<T>;

mod value;
pub use value::Value;

mod machine;
pub use machine::Machine;

mod function;
pub use function::Function;
