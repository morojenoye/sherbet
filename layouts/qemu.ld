MEMORY
{
	FLASH : ORIGIN = 0x80000000, LENGTH = 16K
	RAM : ORIGIN = 0x80004000, LENGTH = 2K
}

PROVIDE( _HART_STACK_SIZE = 512 );
PROVIDE( _HART_COUNT = 1 );

_stack_0 = ALIGN( ORIGIN ( RAM ) + _HART_STACK_SIZE * _HART_COUNT, 8 );
_stack_1 = ORIGIN ( RAM );

ENTRY( start )

SECTIONS
{
	.text : {
		* ( .text.start )
		* ( .rodata .rodata.* )
		* ( .text .text.* )
		. = ALIGN( 8 );
		* ( .text.interrupt )
	} > FLASH

	.data _stack_0 : {
		PROVIDE( __global_pointer$ = . + 128 );
		* ( .sdata .sdata.* )
		* ( .data .data.* )
	} > RAM

	.wipe ( NOLOAD ) : {
		_wipe_0 = .;
		* ( .sbss .sbss.* )
		* ( .bss .bss.* )
		_wipe_1 = .;
	} > RAM
}
