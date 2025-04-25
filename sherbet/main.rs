#![no_std]
use core::{
	arch::{asm, global_asm},
	panic::PanicInfo,
};

global_asm!(
	".section .text.start",
	".globl start",
	"start:",
	"la sp, _stack_0",
	"j setup",
);

#[no_mangle]
unsafe fn setup() -> ! {
	asm!(
		".option push",
		".option norelax",
		"la gp, __global_pointer$",
		".option pop"
	);
	extern "Rust" {
		fn main() -> !;
	}
	main();
	#[allow(unreachable_code)]
	loop {}
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
	loop {}
}
