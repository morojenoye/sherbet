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
	"la sp, _stack_hi",
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

pub unsafe fn reloc() {
	extern "C" {
		static _data_la: u8;
		static mut _data_va: u8;
		static _data_hi: u8;
		static mut _wipe_va: u8;
		static _wipe_hi: u8;
	}
	let hi = &raw const _data_hi;
	let lo = &raw mut _data_va;

	let count = hi.byte_offset_from_unsigned(lo);
	core::ptr::copy_nonoverlapping(
		&raw const _data_la,
		&raw mut _data_va,
		count,
	);
	let hi = &raw const _wipe_hi;
	let lo = &raw mut _wipe_va;

	let count = hi.byte_offset_from_unsigned(lo);
	core::ptr::write_bytes(lo, 0, count);
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
	loop {}
}
