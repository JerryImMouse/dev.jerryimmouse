// Well, its really should be removed when unused, but... I like how it looks.
// Probably should use some cfg_attrs to include that code or not
#![allow(unused)]
pub mod project;
pub mod toaster;

pub use toaster::toast;
