MEMORY
{
	R0 : ORIGIN = 0x80000000, LENGTH = 2K
}

PROVIDE( _HART_STACK_SIZE = 512 );
PROVIDE( _HART_COUNT = 1 );

ENTRY( start )

SECTIONS
{
	.text : {
		* ( .text.start )

		. = ALIGN( 8 );
		_stack_1 = .;
		. = . + _HART_STACK_SIZE * _HART_COUNT;
		. = ALIGN( 8 );
		_stack_0 = .;

		* ( .rodata .rodata.* )
		* ( .text .text.* )

		. = ALIGN( 8 );
		* ( .text.interrupt )
	} > R0

	.data : {
		PROVIDE( __global_pointer$ = . + 128 );
		* ( .sdata .sdata.* )
		* ( .data .data.* )
	} > R0

	.wipe ( NOLOAD ) : {
		_wipe_0 = .;
		* ( .sbss .sbss.* )
		* ( .bss .bss.* )
		_wipe_1 = .;
	} > R0
}
