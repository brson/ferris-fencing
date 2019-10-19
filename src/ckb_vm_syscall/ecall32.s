    # A bunch of trampolines into the ecall function.
    # In Rust these look like
    #
    #     ecallX(syscall: u32, arg0, u32, ...) -> u32
    #
    # The ABI says that arg0 is in a0, arg1 in a1 etc., up
    # to a7, and the return value is in a0.
    # 
    #
    # Ultimately for ecall though the syscall number ends
    # up in a7, so these all end up 'shifting' the argument
    # registers over by one so that ecallX can take the syscall
    # number in the first register.
    #
    # Doesn't touch any callee save registers, including `ra`,
    # the return address, so the system should return back into
    # the original caller.
    #
    # Inefficient but easy to understand.
    
    .text

	.section	.text.__ecall0,"ax",@progbits
	.globl	__ecall0
	.p2align	1
	.type	__ecall0,@function
__ecall0:
    addi a7, a0, 0
    ecall
    ret

	.section	.text.__ecall1,"ax",@progbits
	.globl	__ecall1
	.p2align	1
	.type	__ecall1,@function
__ecall1:
    addi a7, a0, 0
    addi a0, a1, 0
    ecall
    ret

	.section	.text.__ecall2,"ax",@progbits
	.globl	__ecall2
	.p2align	1
	.type	__ecall2,@function
__ecall2:
    addi a7, a0, 0
    addi a0, a1, 0
    addi a1, a2, 0
    ecall
    ret

	.section	.text.__ecall3,"ax",@progbits
	.globl	__ecall3
	.p2align	1
	.type	__ecall3,@function
__ecall3:
    addi a7, a0, 0
    addi a0, a1, 0
    addi a1, a2, 0
    addi a2, a3, 0
    ecall
    ret

	.section	.text.__ecall4,"ax",@progbits
	.globl	__ecall4
	.p2align	1
	.type	__ecall4,@function
__ecall4:
    addi a7, a0, 0
    addi a0, a1, 0
    addi a1, a2, 0
    addi a2, a3, 0
    addi a3, a4, 0
    ecall
    ret

	.section	.text.__ecall5,"ax",@progbits
	.globl	__ecall5
	.p2align	1
	.type	__ecall5,@function
__ecall5:
    addi a7, a0, 0
    addi a0, a1, 0
    addi a1, a2, 0
    addi a2, a3, 0
    addi a3, a4, 0
    addi a4, a5, 0
    ecall
    ret

	.section	.text.__ecall6,"ax",@progbits
	.globl	__ecall6
	.p2align	1
	.type	__ecall,@function
__ecall6:
    addi a7, a0, 0
    addi a0, a1, 0
    addi a1, a2, 0
    addi a2, a3, 0
    addi a3, a4, 0
    addi a4, a5, 0
    addi a5, a6, 0
    ecall
    ret
