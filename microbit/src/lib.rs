#![no_std]
#![no_main]
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]
#![doc = include_str!("../../README.md")]
#![warn(missing_docs)]
mod board;
pub use board::*;

pub mod display;
