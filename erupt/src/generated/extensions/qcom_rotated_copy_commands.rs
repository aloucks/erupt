#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_QCOM_rotated_copy_commands_SPEC_VERSION")]
pub const QCOM_ROTATED_COPY_COMMANDS_SPEC_VERSION: u32 = 0;
#[doc = "<s>Vulkan Manual Page</s> · Constant"]
#[doc(alias = "VK_QCOM_rotated_copy_commands_EXTENSION_NAME")]
pub const QCOM_ROTATED_COPY_COMMANDS_EXTENSION_NAME: *const std::os::raw::c_char =
    crate::cstr!("VK_QCOM_rotated_copy_commands");
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyCommandTransformInfoQCOM.html) · Structure"]
#[doc(alias = "VkCopyCommandTransformInfoQCOM")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CopyCommandTransformInfoQCOM {
    pub s_type: crate::vk1_0::StructureType,
    pub p_next: *const std::ffi::c_void,
    pub transform: crate::extensions::khr_surface::SurfaceTransformFlagBitsKHR,
}
impl Default for CopyCommandTransformInfoQCOM {
    fn default() -> Self {
        Self {
            s_type: crate::vk1_0::StructureType::COPY_COMMAND_TRANSFORM_INFO_QCOM,
            p_next: std::ptr::null(),
            transform: Default::default(),
        }
    }
}
impl std::fmt::Debug for CopyCommandTransformInfoQCOM {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("CopyCommandTransformInfoQCOM")
            .field("s_type", &self.s_type)
            .field("p_next", &self.p_next)
            .field("transform", &self.transform)
            .finish()
    }
}
impl CopyCommandTransformInfoQCOM {
    #[inline]
    pub fn into_builder<'a>(self) -> CopyCommandTransformInfoQCOMBuilder<'a> {
        CopyCommandTransformInfoQCOMBuilder(self, std::marker::PhantomData)
    }
}
#[derive(Copy, Clone)]
#[doc = "[Vulkan Manual Page](https://www.khronos.org/registry/vulkan/specs/1.2-extensions/man/html/VkCopyCommandTransformInfoQCOM.html) · Builder of [`CopyCommandTransformInfoQCOM`]"]
#[repr(transparent)]
pub struct CopyCommandTransformInfoQCOMBuilder<'a>(
    CopyCommandTransformInfoQCOM,
    std::marker::PhantomData<&'a ()>,
);
impl<'a> CopyCommandTransformInfoQCOMBuilder<'a> {
    #[inline]
    pub fn new() -> CopyCommandTransformInfoQCOMBuilder<'a> {
        CopyCommandTransformInfoQCOMBuilder(Default::default(), std::marker::PhantomData)
    }
    #[inline]
    pub fn transform(
        mut self,
        transform: crate::extensions::khr_surface::SurfaceTransformFlagBitsKHR,
    ) -> Self {
        self.0.transform = transform as _;
        self
    }
    #[inline]
    #[doc = "Discards all lifetime information. Use the `Deref` and `DerefMut` implementations if possible."]
    pub fn build(self) -> CopyCommandTransformInfoQCOM {
        self.0
    }
}
impl<'a> std::default::Default for CopyCommandTransformInfoQCOMBuilder<'a> {
    fn default() -> CopyCommandTransformInfoQCOMBuilder<'a> {
        Self::new()
    }
}
impl<'a> std::fmt::Debug for CopyCommandTransformInfoQCOMBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}
impl<'a> std::ops::Deref for CopyCommandTransformInfoQCOMBuilder<'a> {
    type Target = CopyCommandTransformInfoQCOM;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> std::ops::DerefMut for CopyCommandTransformInfoQCOMBuilder<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
