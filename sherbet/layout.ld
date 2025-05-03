_stack_hi = ALIGN( ORIGIN ( RAM ) + _HART_STACK_SIZE * _HART_COUNT, 8 );
_stack_va = ORIGIN ( RAM );

ENTRY( start )

SECTIONS
{
	.text : {
		/* must be single jump instruction */
		KEEP ( * ( .text.start ))

		/* needed in case jump instruction is compressed */
		. = ALIGN( 4 );

		ASSERT (( . == ORIGIN(FLASH) + 4 ),
			"start must be single jump instruction" )

		/* correct absolute address required */
		KEEP ( * ( .text.interrupt ))

		* ( .rodata .rodata.* )
		* ( .text .text.* )
	} >FLASH

	.data _stack_hi : {
		/* addr in flash and in ram */
		_data_la = LOADADDR ( .data );
		_data_va = .;

		__global_pointer$ = . + 128;
		* ( .sdata .sdata.* )
		* ( .data .data.* )
		_data_hi = .;
	} >RAM AT >FLASH

	.wipe ( NOLOAD ) : {
		_wipe_va = .;
		* ( .sbss .sbss.* )
		* ( .bss .bss.* )
		_wipe_hi = .;
	} >RAM
}
