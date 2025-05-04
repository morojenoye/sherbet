#![no_main]
#![no_std]
extern crate sherbet;

use qingke_v2a::interrupt;

#[no_mangle]
extern "Rust" fn sherbet_user_provided_entry() {
	unsafe {
		interrupt::setup();
		sherbet::reloc();
	}
	loop {}
}
