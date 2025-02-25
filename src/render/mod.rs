#[cfg(feature = "debug")]
pub(crate) mod debug;
pub(crate) mod display;
// This technically doesn't need DBus but nothing else implements it atm
#[allow(dead_code)]
pub(crate) mod notifications;
pub mod scheduler;
pub(crate) mod stream;
pub(crate) mod text;
pub(crate) mod util;
#[cfg(feature = "gif")]
pub(crate) mod gif;