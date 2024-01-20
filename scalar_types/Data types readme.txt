Length	     Signed	     Unsigned
8-bit	      i8	         u8
16-bit	     i16	        u16
32-bit	     i32	        u32
64-bit	     i64	        u64
128-bit	     i128	        u128
arch	     isize	        usize

An interger is a number without a fractional component. Lets take the u32 type.
This type declaration indicates that the value it’s associated with should be an unsigned integer (signed integer types start with i instead of u) that takes up 32 bits of space.
Signed and unsigned refer to whether it’s possible for the number to be negative—in other words, whether the number needs to have a sign with it (signed) or whether it will only ever be positive and can therefore be represented without a sign (unsigned)

So an i8 can store numbers from -128 to 127. Unsigned u8 can store numbers 0 to 255

The isize and usize types depends on the archetecture of the computer your program is running on, which the table is denoted as "arch"

Integer literals:

Number literals	    Example
Decimal	            98_222
Hex	                0xff
Octal	            0o77
Binary	            0b1111_0000
Byte (u8 only)	    b'A'

So how do you know which type of integer to use? If you’re unsure, Rust’s defaults are generally good places to start: integer types default to i32.
The primary situation in which you’d use isize or usize is when indexing some sort of collection.
