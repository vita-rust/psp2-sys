#[cfg_attr(
    not(feature = "dox"),
    link(kind = "static", name = "SceLibKernel_stub")
)]
extern "C" {
    pub fn sceKernelGetRandomNumber(output: *mut crate::void, size: u32);
}
