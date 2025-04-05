unsafe extern "C" {
    pub safe fn stext();
    pub safe fn etext();
    pub safe fn srodata();
    pub safe fn erodata();
    pub safe fn sdata();
    pub safe fn edata();
    pub safe fn sbss();
    pub safe fn sbss_with_stack();
    pub safe fn ebss();
    pub safe fn ekernel();
    pub safe fn strampoline();

}
