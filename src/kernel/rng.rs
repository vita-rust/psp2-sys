use crate::types::SceSize;

#[cfg_attr(
    not(feature = "dox"),
    link(kind = "static", name = "SceLibKernel_stub")
)]
extern "C" {
    /// Fills the output buffer with random data.
    ///
    /// * `output` (out) - Output buffer
    /// * `size` - Size of the output buffer, 64 bytes maximum
    ///
    /// Returns 0 on success, < 0 on error.
    pub fn sceKernelGetRandomNumber(output: *mut crate::void, size: SceSize);
}
