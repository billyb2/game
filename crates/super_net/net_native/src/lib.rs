#![feature(explicit_generic_args_with_impl_trait)]

#[cfg(feature = "native")]
mod native;

#[cfg(feature = "native")]
pub use native::*;

#[cfg(feature = "web")]
pub use native_shared::*;
