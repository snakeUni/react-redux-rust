#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(alloc))]

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;
#[cfg(feature = "std")]
use std::vec::Vec;

mod middleware;
mod reducer;
mod store;
mod subscription;

pub use middleware::Middleware;
pub use reducer::Reducer;
#[cfg(not(feature = "devtools"))]
pub use store::Store;
pub use subscription::Subscription;