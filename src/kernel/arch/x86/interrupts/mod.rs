#![feature(naked_functions)]
#![feature(core_intrinsics)]

use core::arch::asm;
use crate::println;

pub mod idt;
pub mod exception;
mod page_fault;
