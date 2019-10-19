#![no_std]

extern "system" {
    #[link_name = "__ecall0"]
    pub fn ecall0(
        num: usize,
    ) -> usize;

    #[link_name = "__ecall1"]
    pub fn ecall1(
        num: usize,
        arg0: usize,
    ) -> usize;

    #[link_name = "__ecall2"]
    pub fn ecall2(
        num: usize,
        arg0: usize,
        arg1: usize,
    ) -> usize;

    #[link_name = "__ecall3"]
    pub fn ecall3(
        num: usize,
        arg0: usize,
        arg1: usize,
        arg2: usize,
    ) -> usize;

    #[link_name = "__ecall4"]
    pub fn ecall4(
        num: usize,
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
    ) -> usize;

    #[link_name = "__ecall5"]
    pub fn ecall5(
        num: usize,
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
    ) -> usize;

    #[link_name = "__ecall6"]
    pub fn ecall6(
        num: usize,
        arg0: usize,
        arg1: usize,
        arg2: usize,
        arg3: usize,
        arg4: usize,
        arg5: usize,
    ) -> usize;
}
