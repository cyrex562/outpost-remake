# Outpost-RS

A port of the old Win 3.1/DOS game Outpost to Rust based on disassembly/decompilation performed in Ghidra


## References

* INT 21 Codes [https://stanislavs.org/helppc/int_21.html]
* general function dispatcher [http://bbc.nvg.org/doc/Master%20512%20Technical%20Guide/m512techb_int21.htm]

## TODO List

* Look through code again in Ghidra at swi(0x21) calls for AH values to determine which functions are getting called and their arguments
