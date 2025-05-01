#![no_main]
#![no_std]
extern crate sherbet;

use qingkev2a::interrupt;

#[no_mangle]
extern "Rust" fn sherbet_user_provided_entry() {
	unsafe { interrupt::setup() };
	loop {}
}
