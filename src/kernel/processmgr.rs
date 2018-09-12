#[cfg_attr(
    not(feature = "dox"),
    link(kind = "static", name = "SceLibKernel_stub")
)]
extern "C" {
    pub fn sceKernelExitProcess(res: i32) -> i32;
}
