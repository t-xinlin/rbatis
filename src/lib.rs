#![allow(unused_imports)]
#![allow(unreachable_patterns)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate lazy_static;
extern crate once_cell;
#[macro_use]
extern crate serde_json;
extern crate serde_yaml;

pub mod ast;
pub mod utils;
pub mod crud;
pub mod convert;
pub mod engine;
pub mod decode;
pub mod plugin;
pub mod rbatis;