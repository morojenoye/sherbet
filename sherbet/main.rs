#![no_std]
use core::{
	arch::{asm, global_asm},
	panic::PanicInfo,
};

pub mod clock;
pub mod interface;

global_asm! {
	".section .text.start",
	"j setup",
}

global_asm! {
	".section .text.setup",
	".globl setup",
	"setup:",
	"la sp, _stack_0",
	"j sherbet",
}

#[no_mangle]
unsafe extern "C" fn sherbet() -> ! {
	asm!(
		".option push",
		".option norelax",
		"la gp, __global_pointer$",
		".option pop"
	);
	extern "Rust" {
		fn sherbet_user_provided_entry();
	}
	sherbet_user_provided_entry();
	loop {}
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
	loop {}
}
