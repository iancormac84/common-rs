#![feature(specialization)]
#![feature(coroutines, coroutine_trait)]

pub extern crate rand;

pub mod ordified;
pub mod mut_rc; 
pub mod easing; 
pub mod math;
pub mod coro;

pub use ordified::*;
pub use mut_rc::*;
pub use easing::*;
pub use math::*;
pub use coro::*;

#[macro_export]
macro_rules! match_pattern {
	($v:expr, $p:pat) => {
		match $v {
			$p => true,
			_ => false,
		}
	}
}
